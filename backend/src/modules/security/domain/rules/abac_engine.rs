use std::collections::HashMap;

/// Attribute-Based Access Control (ABAC) policy engine
/// Implements fine-grained access control based on user attributes,
/// resource attributes, and environmental conditions.
///
/// Reference: OWASP ASVS V4.1 - Access Control Requirements
#[derive(Debug, Clone)]
pub struct AbacPolicyEngine {
    policies: Vec<AbacPolicy>,
}

/// A single ABAC policy rule
#[derive(Debug, Clone)]
pub struct AbacPolicy {
    pub id: String,
    pub name: String,
    pub description: String,
    pub effect: PolicyEffect,
    pub conditions: Vec<PolicyCondition>,
    pub priority: i32,
}

/// Policy effect: allow or deny
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyEffect {
    Allow,
    Deny,
}

/// A single condition in a policy
#[derive(Debug, Clone)]
pub struct PolicyCondition {
    pub attribute: String,
    pub operator: ConditionOperator,
    pub value: String,
}

/// Operators for condition evaluation.
/// All variants are exhaustive — add new operators here and update
/// `evaluate_condition` accordingly.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConditionOperator {
    Equals,
    NotEquals,
    In,
    NotIn,
    Contains,
    StartsWith,
    EndsWith,
}

/// Context for policy evaluation
#[derive(Debug, Clone)]
pub struct AccessContext {
    pub user_id: i64,
    pub user_role: String,
    pub user_permissions: Vec<String>,
    pub branch_id: Option<i64>,
    pub is_company_wide: bool,
    pub resource_type: String,
    pub resource_id: Option<String>,
    pub action: String,
    pub attributes: HashMap<String, String>,
}

/// Result of policy evaluation
#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use = "Access decision must be checked — ignoring it may silently bypass authorization"]
pub enum AccessDecision {
    Allow,
    Deny(String),
}

impl AbacPolicyEngine {
    /// Create a new empty policy engine
    pub fn new() -> Self {
        Self {
            policies: Vec::new(),
        }
    }

    /// Register a policy
    pub fn add_policy(&mut self, policy: AbacPolicy) {
        self.policies.push(policy);
        // Sort by priority (higher priority first)
        self.policies.sort_by_key(|p| std::cmp::Reverse(p.priority));
    }

    /// Evaluate access for the given context
    pub fn evaluate(&self, context: &AccessContext) -> AccessDecision {
        // 1. Check RBAC permissions with wildcard matching
        //    Permission format: "module.resource.action" (e.g. "device.router.view")
        //    We check if any user permission matches the requested resource+action.
        let has_rbac_permission = context
            .user_permissions
            .iter()
            .any(|perm| self.permission_matches(perm, &context.resource_type, &context.action));

        if has_rbac_permission {
            // Check if any ABAC deny policy overrides this RBAC permission
            for policy in &self.policies {
                if policy.effect == PolicyEffect::Deny
                    && self.evaluate_conditions(&policy.conditions, context)
                {
                    return AccessDecision::Deny(format!(
                        "Access denied by policy: {}",
                        policy.name
                    ));
                }
            }
            return AccessDecision::Allow;
        }

        // 2. Evaluate ABAC policies in priority order (deny first)
        for policy in &self.policies {
            if policy.effect == PolicyEffect::Deny
                && self.evaluate_conditions(&policy.conditions, context)
            {
                return AccessDecision::Deny(format!("Access denied by policy: {}", policy.name));
            }
        }

        // 3. Check allow policies
        for policy in &self.policies {
            if policy.effect == PolicyEffect::Allow
                && self.evaluate_conditions(&policy.conditions, context)
            {
                return AccessDecision::Allow;
            }
        }

        // 4. Default deny
        AccessDecision::Deny("No matching policy found".to_string())
    }

    /// Check if a user permission matches a requested resource type and action.
    /// Supports wildcards: "device.*.view" matches "device.router.view"
    /// Permission format: "module.resource.action" or "resource.action"
    fn permission_matches(&self, perm: &str, resource_type: &str, action: &str) -> bool {
        let parts: Vec<&str> = perm.split('.').collect();
        if parts.is_empty() {
            return false;
        }

        match parts.len() {
            // "module.resource.action" format
            // module must match resource_type, action must match.
            // resource is more specific (e.g. "router" within "device" module) and is
            // checked at a finer granularity — for basic RBAC we only verify module+action.
            3 => {
                let module = parts[0];
                let perm_action = parts[2];
                (module == "*" || module == resource_type)
                    && (perm_action == "*" || perm_action == action)
            }
            // "resource.action" format
            2 => {
                let resource = parts[0];
                let perm_action = parts[1];
                (resource == "*" || resource == resource_type)
                    && (perm_action == "*" || perm_action == action)
            }
            1 => {
                let single = parts[0];
                single == "*" || single == resource_type || single == action
            }
            _ => false,
        }
    }

    /// Evaluate a set of conditions against the context (AND logic)
    fn evaluate_conditions(&self, conditions: &[PolicyCondition], context: &AccessContext) -> bool {
        conditions
            .iter()
            .all(|condition| self.evaluate_condition(condition, context))
    }

    /// Evaluate a single condition
    fn evaluate_condition(&self, condition: &PolicyCondition, context: &AccessContext) -> bool {
        let attribute_value = match condition.attribute.as_str() {
            "user.role" => Some(&context.user_role),
            "resource.type" => Some(&context.resource_type),
            "resource.id" => context.resource_id.as_ref(),
            "action" => Some(&context.action),
            "user.is_company_wide" => {
                return context.is_company_wide == condition.value.parse().unwrap_or(false)
            }
            "branch.id" => {
                return match &context.branch_id {
                    Some(id) => match condition.operator {
                        ConditionOperator::Equals => id.to_string() == condition.value,
                        ConditionOperator::In => condition
                            .value
                            .split(',')
                            .any(|v| v.trim() == id.to_string()),
                        _ => false,
                    },
                    None => false,
                }
            }
            _ => {
                // Check custom attributes
                match context.attributes.get(&condition.attribute) {
                    Some(val) => {
                        return match condition.operator {
                            ConditionOperator::Equals => val == &condition.value,
                            ConditionOperator::NotEquals => val != &condition.value,
                            ConditionOperator::In => {
                                condition.value.split(',').any(|v| v.trim() == val.as_str())
                            }
                            ConditionOperator::NotIn => {
                                !condition.value.split(',').any(|v| v.trim() == val.as_str())
                            }
                            ConditionOperator::Contains => val.contains(&condition.value),
                            ConditionOperator::StartsWith => val.starts_with(&condition.value),
                            ConditionOperator::EndsWith => val.ends_with(&condition.value),
                        };
                    }
                    None => return false,
                }
            }
        };

        let val = match attribute_value {
            Some(v) => v.as_str(),
            None => return false,
        };

        match condition.operator {
            ConditionOperator::Equals => val == condition.value,
            ConditionOperator::NotEquals => val != condition.value,
            ConditionOperator::In => condition.value.split(',').any(|v| v.trim() == val),
            ConditionOperator::NotIn => !condition.value.split(',').any(|v| v.trim() == val),
            ConditionOperator::Contains => val.contains(&condition.value),
            ConditionOperator::StartsWith => val.starts_with(&condition.value),
            ConditionOperator::EndsWith => val.ends_with(&condition.value),
        }
    }

    /// Create default policies for AeroXe
    pub fn with_default_policies() -> Self {
        let mut engine = Self::new();

        // Deny: Support agents cannot modify bandwidth directly (must go through NOC)
        // This prevents accidental bandwidth changes by support staff.
        engine.add_policy(AbacPolicy {
            id: "abac-001".to_string(),
            name: "support-bandwidth-restriction".to_string(),
            description: "Support agents and NOC engineers cannot directly modify bandwidth — must use escalation workflow"
                .to_string(),
            effect: PolicyEffect::Deny,
            conditions: vec![
                PolicyCondition {
                    attribute: "user.role".to_string(),
                    operator: ConditionOperator::In,
                    value: "customer_support,noc_engineer".to_string(),
                },
                PolicyCondition {
                    attribute: "resource.type".to_string(),
                    operator: ConditionOperator::Equals,
                    value: "bandwidth".to_string(),
                },
                PolicyCondition {
                    attribute: "action".to_string(),
                    operator: ConditionOperator::Equals,
                    value: "update".to_string(),
                },
            ],
            priority: 100,
        });

        // Deny: Customer self-service cannot perform write operations on other customers' data
        engine.add_policy(AbacPolicy {
            id: "abac-002".to_string(),
            name: "customer-write-restriction".to_string(),
            description: "Customers cannot perform write operations on billing or subscription data via self-service".to_string(),
            effect: PolicyEffect::Deny,
            conditions: vec![
                PolicyCondition {
                    attribute: "user.role".to_string(),
                    operator: ConditionOperator::Equals,
                    value: "customer".to_string(),
                },
                PolicyCondition {
                    attribute: "resource.type".to_string(),
                    operator: ConditionOperator::In,
                    value: "invoice,refund,discount,payment".to_string(),
                },
                PolicyCondition {
                    attribute: "action".to_string(),
                    operator: ConditionOperator::In,
                    value: "create,update,delete,process,refund".to_string(),
                },
            ],
            priority: 200,
        });

        // Deny: Field technicians cannot access billing
        engine.add_policy(AbacPolicy {
            id: "abac-003".to_string(),
            name: "technician-billing-restriction".to_string(),
            description: "Field technicians cannot access billing data".to_string(),
            effect: PolicyEffect::Deny,
            conditions: vec![
                PolicyCondition {
                    attribute: "user.role".to_string(),
                    operator: ConditionOperator::Equals,
                    value: "field_technician".to_string(),
                },
                PolicyCondition {
                    attribute: "resource.type".to_string(),
                    operator: ConditionOperator::In,
                    value: "invoice,payment,refund,discount".to_string(),
                },
            ],
            priority: 300,
        });

        // Deny: Billing operators cannot modify network config
        engine.add_policy(AbacPolicy {
            id: "abac-004".to_string(),
            name: "billing-network-separation".to_string(),
            description: "Billing operators cannot modify network configuration".to_string(),
            effect: PolicyEffect::Deny,
            conditions: vec![
                PolicyCondition {
                    attribute: "user.role".to_string(),
                    operator: ConditionOperator::Equals,
                    value: "billing_operator".to_string(),
                },
                PolicyCondition {
                    attribute: "resource.type".to_string(),
                    operator: ConditionOperator::In,
                    value: "device,vlan,ip_pool,bandwidth".to_string(),
                },
                PolicyCondition {
                    attribute: "action".to_string(),
                    operator: ConditionOperator::In,
                    value: "create,update,delete,configure".to_string(),
                },
            ],
            priority: 300,
        });

        engine
    }
}

impl Default for AbacPolicyEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_context(
        role: &str,
        permissions: Vec<&str>,
        action: &str,
        resource: &str,
    ) -> AccessContext {
        AccessContext {
            user_id: 1,
            user_role: role.to_string(),
            user_permissions: permissions.into_iter().map(String::from).collect(),
            branch_id: Some(1),
            is_company_wide: matches!(role, "super_admin" | "isp_owner" | "finance_manager"),
            resource_type: resource.to_string(),
            resource_id: None,
            action: action.to_string(),
            attributes: HashMap::new(),
        }
    }

    #[test]
    fn test_rbac_permission_allows_access() {
        let engine = AbacPolicyEngine::new();
        let ctx = make_context("noc_engineer", vec!["device.router.view"], "view", "device");
        assert_eq!(engine.evaluate(&ctx), AccessDecision::Allow);
    }

    #[test]
    fn test_rbac_wildcard_permission() {
        let engine = AbacPolicyEngine::new();
        let ctx = make_context("network_admin", vec!["device.*.view"], "view", "device");
        assert_eq!(engine.evaluate(&ctx), AccessDecision::Allow);
    }

    #[test]
    fn test_default_deny_without_permission() {
        let engine = AbacPolicyEngine::new();
        let ctx = make_context("customer_support", vec![], "configure", "device");
        assert!(matches!(engine.evaluate(&ctx), AccessDecision::Deny(_)));
    }

    #[test]
    fn test_abac_technician_cannot_access_billing() {
        let engine = AbacPolicyEngine::with_default_policies();
        let ctx = make_context(
            "field_technician",
            vec!["invoice.view", "invoice.create"],
            "create",
            "invoice",
        );
        assert!(matches!(engine.evaluate(&ctx), AccessDecision::Deny(_)));
    }

    #[test]
    fn test_abac_billing_operator_cannot_modify_device() {
        let engine = AbacPolicyEngine::with_default_policies();
        let ctx = make_context(
            "billing_operator",
            vec!["device.router.configure"],
            "configure",
            "device",
        );
        assert!(matches!(engine.evaluate(&ctx), AccessDecision::Deny(_)));
    }

    #[test]
    fn test_policy_priority_deny_overrides_allow() {
        let mut engine = AbacPolicyEngine::new();
        engine.add_policy(AbacPolicy {
            id: "allow-test".to_string(),
            name: "allow-test".to_string(),
            description: "Allow all".to_string(),
            effect: PolicyEffect::Allow,
            conditions: vec![PolicyCondition {
                attribute: "user.role".to_string(),
                operator: ConditionOperator::Equals,
                value: "test_role".to_string(),
            }],
            priority: 10,
        });
        engine.add_policy(AbacPolicy {
            id: "deny-test".to_string(),
            name: "deny-test".to_string(),
            description: "Deny specific".to_string(),
            effect: PolicyEffect::Deny,
            conditions: vec![PolicyCondition {
                attribute: "resource.type".to_string(),
                operator: ConditionOperator::Equals,
                value: "secret_resource".to_string(),
            }],
            priority: 200,
        });
        let ctx = make_context(
            "test_role",
            vec!["secret_resource.*.access"],
            "access",
            "secret_resource",
        );
        assert!(matches!(engine.evaluate(&ctx), AccessDecision::Deny(_)));
    }

    #[test]
    fn test_branch_scoped_condition() {
        let engine = AbacPolicyEngine::with_default_policies();
        let mut ctx = make_context("noc_engineer", vec!["device.router.view"], "view", "device");
        ctx.branch_id = Some(1);
        assert_eq!(engine.evaluate(&ctx), AccessDecision::Allow);
    }

    #[test]
    fn test_custom_attribute_condition() {
        let mut engine = AbacPolicyEngine::new();
        engine.add_policy(AbacPolicy {
            id: "attr-test".to_string(),
            name: "attr-test".to_string(),
            description: "Allow based on custom attribute".to_string(),
            effect: PolicyEffect::Allow,
            conditions: vec![PolicyCondition {
                attribute: "customer.status".to_string(),
                operator: ConditionOperator::Equals,
                value: "active".to_string(),
            }],
            priority: 100,
        });
        let mut ctx = make_context("test_role", vec![], "view", "subscription");
        ctx.attributes
            .insert("customer.status".to_string(), "active".to_string());
        assert_eq!(engine.evaluate(&ctx), AccessDecision::Allow);
    }
}

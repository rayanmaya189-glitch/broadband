-- Widen subscription.review_status to accept 'pending_downgrade', the flag
-- written by SubscriptionService::downgrade_subscription() to mark a soft
-- downgrade for review before it takes effect at the next billing cycle.
ALTER TABLE subscription.subscriptions DROP CONSTRAINT IF EXISTS subscriptions_review_status_check;
ALTER TABLE subscription.subscriptions ADD CONSTRAINT subscriptions_review_status_check CHECK (
    review_status IN ('pending', 'approved', 'rejected', 'pending_downgrade')
);

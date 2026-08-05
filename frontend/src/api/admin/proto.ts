// ─── Minimal protobuf wire-format helpers + AeroXe customer messages ──────
// Only used for the customer module (the one protobuf API in the backend).

const enc = new TextEncoder();
const dec = new TextDecoder();

export interface ProtoField {
  field: number;
  wire: number;
  value: number | Uint8Array;
}

export function encodeVarint(n: number): number[] {
  let v = BigInt(Math.trunc(n));
  const bytes: number[] = [];
  for (;;) {
    const byte = Number(v & 0x7fn);
    v >>= 7n;
    if (v === 0n) {
      bytes.push(byte);
      return bytes;
    }
    bytes.push(byte | 0x80);
  }
}

function tag(field: number, wire: number): number[] {
  return encodeVarint((field << 3) | wire);
}

export function writeString(field: number, value: string): number[] {
  const bytes = enc.encode(value);
  return [...tag(field, 2), ...encodeVarint(bytes.length), ...Array.from(bytes)];
}

export function writeVarint(field: number, value: number): number[] {
  return [...tag(field, 0), ...encodeVarint(value)];
}

export function writeMessage(field: number, bytes: number[]): number[] {
  return [...tag(field, 2), ...encodeVarint(bytes.length), ...bytes];
}

export function writeOptionalString(field: number, value?: string | null): number[] {
  if (value === undefined || value === null || value === '') return [];
  return writeString(field, value);
}

function readVarint(bytes: Uint8Array, offset: number): { value: number; length: number } {
  let result = 0n;
  let shift = 0n;
  let i = offset;
  for (;;) {
    if (i >= bytes.length) throw new Error('Truncated varint');
    const b = bytes[i++];
    result |= BigInt(b & 0x7f) << shift;
    if ((b & 0x80) === 0) break;
    shift += 7n;
    if (shift > 63n) throw new Error('Varint too long');
  }
  return { value: Number(result), length: i - offset };
}

export function parseProto(bytes: Uint8Array): ProtoField[] {
  const fields: ProtoField[] = [];
  let offset = 0;
  while (offset < bytes.length) {
    const { value: tagValue, length: tagLen } = readVarint(bytes, offset);
    offset += tagLen;
    const field = tagValue >>> 3;
    const wire = tagValue & 0x7;
    if (wire === 0) {
      const r = readVarint(bytes, offset);
      offset += r.length;
      fields.push({ field, wire, value: r.value });
    } else if (wire === 2) {
      const r = readVarint(bytes, offset);
      offset += r.length;
      const len = r.value;
      fields.push({ field, wire, value: bytes.slice(offset, offset + len) });
      offset += len;
    } else if (wire === 1) {
      fields.push({ field, wire, value: bytes.slice(offset, offset + 8) });
      offset += 8;
    } else if (wire === 5) {
      fields.push({ field, wire, value: bytes.slice(offset, offset + 4) });
      offset += 4;
    } else {
      throw new Error(`Unsupported wire type ${wire}`);
    }
  }
  return fields;
}

function getString(fields: ProtoField[], field: number): string | undefined {
  const f = fields.find((x) => x.field === field && x.wire === 2);
  if (!f || typeof f.value === 'number') return undefined;
  return dec.decode(f.value as Uint8Array);
}

function getNumber(fields: ProtoField[], field: number): number | undefined {
  const f = fields.find((x) => x.field === field && x.wire === 0);
  if (!f || typeof f.value !== 'number') return undefined;
  return f.value;
}

function getMessage(fields: ProtoField[], field: number): Uint8Array | undefined {
  const f = fields.find((x) => x.field === field && x.wire === 2);
  if (!f || typeof f.value === 'number') return undefined;
  return f.value as Uint8Array;
}

function getMessages(fields: ProtoField[], field: number): Uint8Array[] {
  return fields
    .filter((x) => x.field === field && x.wire === 2 && typeof x.value !== 'number')
    .map((x) => x.value as Uint8Array);
}

// ─── common.Response envelope ───────────────────────────────────────────────

export interface ProtoEnvelope {
  status: number;
  data: Uint8Array | null;
  errorMessage: string | null;
}

export function decodeEnvelope(bytes: Uint8Array): ProtoEnvelope {
  const fields = parseProto(bytes);
  const status = getNumber(fields, 1) ?? 0;
  const data = getMessage(fields, 2) ?? null;
  const errorFields = getMessage(fields, 3);
  let errorMessage: string | null = null;
  if (errorFields) {
    errorMessage = getString(parseProto(errorFields), 2) ?? 'Request failed';
  }
  return { status, data, errorMessage };
}

// ─── PaginationRequest ──────────────────────────────────────────────────────

export interface ProtoPagination {
  page?: number;
  page_size?: number;
  sort_by?: string;
  sort_order?: string;
  cursor?: string;
}

export function encodePagination(p: ProtoPagination | undefined): number[] {
  if (!p) return [];
  const out: number[] = [];
  if (p.page !== undefined) out.push(...writeVarint(1, p.page));
  if (p.page_size !== undefined) out.push(...writeVarint(2, p.page_size));
  out.push(...writeOptionalString(3, p.sort_by));
  out.push(...writeOptionalString(4, p.sort_order));
  out.push(...writeOptionalString(5, p.cursor));
  return out;
}

// ─── ListCustomersRequest / ListCustomersResponse ───────────────────────────

export function encodeListCustomersRequest(opts: {
  page?: number;
  page_size?: number;
  status?: string;
  branch_id?: number;
}): number[] {
  const pagination = encodePagination({ page: opts.page, page_size: opts.page_size });
  const out: number[] = [];
  if (pagination.length) out.push(...writeMessage(1, pagination));
  out.push(...writeOptionalString(2, opts.status));
  if (opts.branch_id !== undefined && opts.branch_id !== null) {
    out.push(...writeVarint(3, opts.branch_id));
  }
  return out;
}

export interface ProtoCustomer {
  id: number;
  customer_code?: string;
  branch_id: number;
  name: string;
  email?: string;
  phone: string;
  status: string;
  created_at?: string;
  updated_at?: string;
}

export function decodeCustomer(bytes: Uint8Array): ProtoCustomer {
  const f = parseProto(bytes);
  return {
    id: getNumber(f, 1) ?? 0,
    customer_code: getString(f, 2),
    branch_id: getNumber(f, 3) ?? 0,
    name: getString(f, 4) ?? '',
    email: getString(f, 5),
    phone: getString(f, 6) ?? '',
    status: getString(f, 7) ?? 'active',
    created_at: getString(f, 8),
    updated_at: getString(f, 9),
  };
}

export interface ListCustomersResult {
  customers: ProtoCustomer[];
  total_count: number;
  page: number;
  page_size: number;
}

export function decodeListCustomersResponse(bytes: Uint8Array): ListCustomersResult {
  const f = parseProto(bytes);
  const customers = getMessages(f, 1).map(decodeCustomer);
  return {
    customers,
    total_count: getNumber(f, 2) ?? 0,
    page: getNumber(f, 3) ?? 1,
    page_size: getNumber(f, 4) ?? 20,
  };
}

// ─── CreateCustomerRequest ───────────────────────────────────────────────────

export function encodeCreateCustomerRequest(req: {
  branch_id: number;
  name: string;
  email?: string;
  phone: string;
  alternate_phone?: string;
}): number[] {
  return [
    ...writeVarint(1, req.branch_id),
    ...writeString(2, req.name),
    ...writeOptionalString(3, req.email),
    ...writeString(4, req.phone),
    ...writeOptionalString(5, req.alternate_phone),
  ];
}

// ─── GetCustomerRequest ─────────────────────────────────────────────────────

export function encodeGetCustomerRequest(customerId: number): number[] {
  return writeVarint(1, customerId);
}

// ─── UpdateCustomerRequest ──────────────────────────────────────────────────

export function encodeUpdateCustomerRequest(req: {
  customer_id: number;
  name?: string;
  email?: string;
  phone?: string;
  alternate_phone?: string;
}): number[] {
  return [
    ...writeVarint(1, req.customer_id),
    ...writeOptionalString(2, req.name),
    ...writeOptionalString(3, req.email),
    ...writeOptionalString(4, req.phone),
    ...writeOptionalString(5, req.alternate_phone),
  ];
}

// ─── UpdateCustomerStatusRequest ────────────────────────────────────────────

export function encodeUpdateCustomerStatusRequest(customerId: number, status: string): number[] {
  return [...writeVarint(1, customerId), ...writeString(2, status)];
}

// ─── DeleteCustomerRequest ──────────────────────────────────────────────────

export function encodeDeleteCustomerRequest(customerId: number): number[] {
  return writeVarint(1, customerId);
}

// ─── SearchCustomersRequest ─────────────────────────────────────────────────

export function encodeSearchCustomersRequest(req: { query?: string; status?: string }): number[] {
  return [
    ...writeOptionalString(1, req.query),
    ...writeOptionalString(2, req.status),
  ];
}

// ─── CustomerAddress / ListAddressesResponse ────────────────────────────────

export interface ProtoCustomerAddress {
  id: number;
  customer_id: number;
  address_type?: string;
  line1: string;
  line2?: string;
  city: string;
  state: string;
  pincode: string;
  is_primary: boolean;
}

export function decodeCustomerAddress(bytes: Uint8Array): ProtoCustomerAddress {
  const f = parseProto(bytes);
  return {
    id: getNumber(f, 1) ?? 0,
    customer_id: getNumber(f, 2) ?? 0,
    address_type: getString(f, 3),
    line1: getString(f, 4) ?? '',
    line2: getString(f, 5),
    city: getString(f, 6) ?? '',
    state: getString(f, 7) ?? '',
    pincode: getString(f, 8) ?? '',
    is_primary: (getNumber(f, 9) ?? 0) === 1,
  };
}

export function decodeListAddressesResponse(bytes: Uint8Array): ProtoCustomerAddress[] {
  const f = parseProto(bytes);
  return getMessages(f, 1).map(decodeCustomerAddress);
}

// ─── ListAddressesRequest ───────────────────────────────────────────────────

export function encodeListAddressesRequest(customerId: number): number[] {
  return writeVarint(1, customerId);
}

// ─── CustomerHistory ────────────────────────────────────────────────────────

export interface ProtoHistoryEntry {
  id: number;
  entity_id?: string;
  action: string;
  old_data?: string;
  new_data?: string;
  changed_fields?: string;
  user_id?: number;
  user_name?: string;
  user_email?: string;
  created_at?: string;
}

export function decodeHistoryEntry(bytes: Uint8Array): ProtoHistoryEntry {
  const f = parseProto(bytes);
  return {
    id: getNumber(f, 1) ?? 0,
    entity_id: getString(f, 2),
    action: getString(f, 3) ?? '',
    old_data: getString(f, 4),
    new_data: getString(f, 5),
    changed_fields: getString(f, 6),
    user_id: getNumber(f, 7),
    user_name: getString(f, 8),
    user_email: getString(f, 9),
    created_at: getString(f, 10),
  };
}

export function decodeCustomerHistoryResponse(bytes: Uint8Array): {
  entity_type: string;
  entity_id: string;
  total_count: number;
  items: ProtoHistoryEntry[];
} {
  const f = parseProto(bytes);
  return {
    entity_type: getString(f, 1) ?? '',
    entity_id: getString(f, 2) ?? '',
    total_count: getNumber(f, 3) ?? 0,
    items: getMessages(f, 4).map(decodeHistoryEntry),
  };
}

export function encodeGetCustomerHistoryRequest(customerId: number, page?: number, page_size?: number): number[] {
  const out: number[] = [...writeVarint(1, customerId)];
  if (page !== undefined) out.push(...writeVarint(2, page));
  if (page_size !== undefined) out.push(...writeVarint(3, page_size));
  return out;
}

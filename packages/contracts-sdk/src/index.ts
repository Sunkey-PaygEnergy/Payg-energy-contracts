import { Buffer } from "buffer";
import { Address } from '@stellar/stellar-sdk';
import {
  AssembledTransaction,
  Client as ContractClient,
  ClientOptions as ContractClientOptions,
  Result,
  Spec as ContractSpec,
} from '@stellar/stellar-sdk/contract';
import type {
  u32,
  i32,
  u64,
  i64,
  u128,
  i128,
  u256,
  i256,
  Option,
  Typepoint,
  Duration,
} from '@stellar/stellar-sdk/contract';
export * from '@stellar/stellar-sdk'
export * as contract from '@stellar/stellar-sdk/contract'
export * as rpc from '@stellar/stellar-sdk/rpc'

if (typeof window !== 'undefined') {
  //@ts-ignore Buffer exists
  window.Buffer = window.Buffer || Buffer;
}


export const networks = {
  testnet: {
    networkPassphrase: "Test SDF Network ; September 2015",
    contractId: "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC",
  }
} as const


export interface Plan {
  active: boolean;
  created_at: u64;
  daily_rate: i128;
  deposit_amount: i128;
  grace_period_seconds: u64;
  min_payment: i128;
  name: string;
  operator: string;
  plan_id: u32;
  token: string;
  total_price: i128;
}


export interface Lease {
  created_at: u64;
  customer: string;
  device_id: Buffer;
  emergency_paused_until: u64;
  last_payment_at: u64;
  lease_id: u64;
  operator: string;
  paid_until: u64;
  plan_id: u32;
  pool_id: Option<u64>;
  status: LeaseStatus;
  total_paid: i128;
}

export type DataKey = {tag: "Admin", values: void} | {tag: "OperatorCount", values: void} | {tag: "PlanCount", values: void} | {tag: "LeaseCount", values: void} | {tag: "PoolCount", values: void} | {tag: "Operator", values: readonly [string]} | {tag: "Plan", values: readonly [u32]} | {tag: "Lease", values: readonly [u64]} | {tag: "Device", values: readonly [Buffer]} | {tag: "Pool", values: readonly [u64]} | {tag: "PoolFinancier", values: readonly [u64, string]} | {tag: "PendingPlanChange", values: readonly [u64]};


export interface Operator {
  active: boolean;
  address: string;
  name: string;
  payout_address: string;
  registered_at: u64;
  total_leases: u64;
  total_volume_collected: i128;
}

export enum AccessState {
  Active = 1,
  GracePeriod = 2,
  Locked = 3,
  Owned = 4,
  Suspended = 5,
  Repossessed = 6,
  Paused = 7,
}

export enum LeaseStatus {
  Active = 1,
  Suspended = 2,
  Repossessed = 3,
  Owned = 4,
}


export interface AccessStatus {
  current_time: u64;
  days_remaining: u32;
  is_repossessable: boolean;
  is_unlocked: boolean;
  lease_id: u64;
  paid_until: u64;
  remaining_to_own: i128;
  seconds_remaining: u64;
  state: AccessState;
  total_paid: i128;
  total_price: i128;
}


export interface FinancierPool {
  acc_reward_per_share: i128;
  created_at: u64;
  funded_amount: i128;
  is_closed: boolean;
  name: string;
  operator: string;
  pool_id: u64;
  repayment_bps: u32;
  target_amount: i128;
  token: string;
  total_repaid: i128;
}


export interface PoolFinancier {
  claimed_amount: i128;
  deposit_amount: i128;
  financier: string;
  pool_id: u64;
  reward_debt: i128;
}


export interface BatchLeaseInput {
  customer: string;
  device_id: Buffer;
  lease_id: u64;
  plan_id: u32;
  pool_id: Option<u64>;
}

export const Errors = {
  1: {message:"AlreadyInitialized"},

  2: {message:"NotInitialized"},

  3: {message:"Unauthorized"},

  4: {message:"OperatorNotFound"},

  5: {message:"OperatorAlreadyExists"},

  6: {message:"OperatorInactive"},

  7: {message:"PlanNotFound"},

  8: {message:"PlanAlreadyExists"},

  9: {message:"PlanInactive"},

  10: {message:"LeaseNotFound"},

  11: {message:"LeaseAlreadyExists"},

  12: {message:"LeaseNotActive"},

  13: {message:"LeaseAlreadyOwned"},

  14: {message:"LeaseSuspended"},

  15: {message:"LeaseRepossessed"},

  16: {message:"InvalidAmount"},

  17: {message:"BelowMinimumPayment"},

  18: {message:"OverpaymentExceedsBalance"},

  19: {message:"RepossessionNotAllowed"},

  20: {message:"GracePeriodActive"},

  21: {message:"PoolNotFound"},

  22: {message:"PoolAlreadyExists"},

  23: {message:"PoolFullyFunded"},

  24: {message:"PoolFundingTargetExceeded"},

  25: {message:"NoEarningsToClaim"},

  26: {message:"DeviceAlreadyAssigned"},

  27: {message:"InvalidPlanParams"},

  28: {message:"InvalidBasisPoints"},

  29: {message:"PlanChangePending"},

  30: {message:"NoPendingPlanChange"},

  31: {message:"PlanChangeMismatch"},

  32: {message:"BatchSizeExceeded"},

  33: {message:"ArithmeticOverflow"},

  34: {message:"ZeroAddress"},

  35: {message:"PauseDurationInvalid"}
}

export interface Client {
  /**
   * Construct and simulate a pay transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  pay: ({payer, lease_id, amount}: {payer: string, lease_id: u64, amount: i128}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<i128>>>

  /**
   * Construct and simulate a version transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  version: (options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<u32>>

  /**
   * Construct and simulate a get_plan transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  get_plan: ({plan_id}: {plan_id: u32}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<Plan>>>

  /**
   * Construct and simulate a get_pool transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  get_pool: ({pool_id}: {pool_id: u64}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<FinancierPool>>>

  /**
   * Construct and simulate a fund_pool transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  fund_pool: ({financier, pool_id, amount}: {financier: string, pool_id: u64, amount: i128}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a get_admin transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  get_admin: (options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<string>>>

  /**
   * Construct and simulate a get_lease transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  get_lease: ({lease_id}: {lease_id: u64}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<Lease>>>

  /**
   * Construct and simulate a is_active transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  is_active: ({lease_id}: {lease_id: u64}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<boolean>>

  /**
   * Construct and simulate a repossess transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  repossess: ({operator, lease_id, reason}: {operator: string, lease_id: u64, reason: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a set_admin transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  set_admin: ({new_admin}: {new_admin: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a get_access transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  get_access: ({lease_id}: {lease_id: u64}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<AccessStatus>>>

  /**
   * Construct and simulate a initialize transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  initialize: ({admin}: {admin: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a create_plan transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  create_plan: ({operator, plan_id, name, token, daily_rate, total_price, deposit_amount, min_payment, grace_period_seconds}: {operator: string, plan_id: u32, name: string, token: string, daily_rate: i128, total_price: i128, deposit_amount: i128, min_payment: i128, grace_period_seconds: u64}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a create_pool transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  create_pool: ({operator, pool_id, token, name, target_amount, repayment_bps}: {operator: string, pool_id: u64, token: string, name: string, target_amount: i128, repayment_bps: u32}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a device_swap transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  device_swap: ({operator, lease_id, new_device_id, reason}: {operator: string, lease_id: u64, new_device_id: Buffer, reason: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a create_lease transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  create_lease: ({operator, lease_id, customer, plan_id, device_id, pool_id}: {operator: string, lease_id: u64, customer: string, plan_id: u32, device_id: Buffer, pool_id: Option<u64>}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a get_operator transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  get_operator: ({operator}: {operator: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<Operator>>>

  /**
   * Construct and simulate a grant_credit transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  grant_credit: ({operator, lease_id, days, reason}: {operator: string, lease_id: u64, days: u32, reason: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<u64>>>

  /**
   * Construct and simulate a set_suspended transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  set_suspended: ({operator, lease_id, suspended, reason}: {operator: string, lease_id: u64, suspended: boolean, reason: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a emergency_pause transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  emergency_pause: ({operator, lease_id, pause_duration_seconds, reason}: {operator: string, lease_id: u64, pause_duration_seconds: u64, reason: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<u64>>>

  /**
   * Construct and simulate a set_plan_active transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  set_plan_active: ({operator, plan_id, active}: {operator: string, plan_id: u32, active: boolean}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a get_device_lease transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  get_device_lease: ({device_id}: {device_id: Buffer}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<u64>>>

  /**
   * Construct and simulate a register_operator transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  register_operator: ({operator, name, payout_address}: {operator: string, name: string, payout_address: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a accept_plan_change transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  accept_plan_change: ({customer, lease_id}: {customer: string, lease_id: u64}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a get_pool_financier transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  get_pool_financier: ({pool_id, financier}: {pool_id: u64, financier: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<PoolFinancier>>>

  /**
   * Construct and simulate a is_device_assigned transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  is_device_assigned: ({device_id}: {device_id: Buffer}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<boolean>>

  /**
   * Construct and simulate a batch_create_leases transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  batch_create_leases: ({operator, leases}: {operator: string, leases: Array<BatchLeaseInput>}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<u32>>>

  /**
   * Construct and simulate a claim_pool_earnings transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  claim_pool_earnings: ({financier, pool_id}: {financier: string, pool_id: u64}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<i128>>>

  /**
   * Construct and simulate a request_plan_change transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  request_plan_change: ({operator, lease_id, new_plan_id}: {operator: string, lease_id: u64, new_plan_id: u32}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a set_operator_active transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  set_operator_active: ({operator, active}: {operator: string, active: boolean}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a update_operator_payout transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  update_operator_payout: ({operator, new_payout}: {operator: string, new_payout: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a get_pending_plan_change transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  get_pending_plan_change: ({lease_id}: {lease_id: u64}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<u32>>>

  /**
   * Construct and simulate a transfer_lease_customer transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  transfer_lease_customer: ({current_customer, new_customer, lease_id}: {current_customer: string, new_customer: string, lease_id: u64}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a calculate_access_seconds transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  calculate_access_seconds: ({amount, daily_rate}: {amount: i128, daily_rate: i128}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<u64>>

  /**
   * Construct and simulate a get_claimable_pool_earnings transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  get_claimable_pool_earnings: ({pool_id, financier}: {pool_id: u64, financier: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<i128>>>

}
export class Client extends ContractClient {
  constructor(public readonly options: ContractClientOptions) {
    super(
      new ContractSpec([ "AAAAAAAAAAAAAAADcGF5AAAAAAMAAAAAAAAABXBheWVyAAAAAAAAEwAAAAAAAAAIbGVhc2VfaWQAAAAGAAAAAAAAAAZhbW91bnQAAAAAAAsAAAABAAAD6QAAAAsAAAAD",
        "AAAAAAAAAAAAAAAHdmVyc2lvbgAAAAAAAAAAAQAAAAQ=",
        "AAAAAAAAAAAAAAAIZ2V0X3BsYW4AAAABAAAAAAAAAAdwbGFuX2lkAAAAAAQAAAABAAAD6QAAB9AAAAAEUGxhbgAAAAM=",
        "AAAAAAAAAAAAAAAIZ2V0X3Bvb2wAAAABAAAAAAAAAAdwb29sX2lkAAAAAAYAAAABAAAD6QAAB9AAAAANRmluYW5jaWVyUG9vbAAAAAAAAAM=",
        "AAAAAAAAAAAAAAAJZnVuZF9wb29sAAAAAAAAAwAAAAAAAAAJZmluYW5jaWVyAAAAAAAAEwAAAAAAAAAHcG9vbF9pZAAAAAAGAAAAAAAAAAZhbW91bnQAAAAAAAsAAAABAAAD6QAAA+0AAAAAAAAAAw==",
        "AAAAAAAAAAAAAAAJZ2V0X2FkbWluAAAAAAAAAAAAAAEAAAPpAAAAEwAAAAM=",
        "AAAAAAAAAAAAAAAJZ2V0X2xlYXNlAAAAAAAAAQAAAAAAAAAIbGVhc2VfaWQAAAAGAAAAAQAAA+kAAAfQAAAABUxlYXNlAAAAAAAAAw==",
        "AAAAAAAAAAAAAAAJaXNfYWN0aXZlAAAAAAAAAQAAAAAAAAAIbGVhc2VfaWQAAAAGAAAAAQAAAAE=",
        "AAAAAAAAAAAAAAAJcmVwb3NzZXNzAAAAAAAAAwAAAAAAAAAIb3BlcmF0b3IAAAATAAAAAAAAAAhsZWFzZV9pZAAAAAYAAAAAAAAABnJlYXNvbgAAAAAAEAAAAAEAAAPpAAAD7QAAAAAAAAAD",
        "AAAAAAAAAAAAAAAJc2V0X2FkbWluAAAAAAAAAQAAAAAAAAAJbmV3X2FkbWluAAAAAAAAEwAAAAEAAAPpAAAD7QAAAAAAAAAD",
        "AAAAAAAAAAAAAAAKZ2V0X2FjY2VzcwAAAAAAAQAAAAAAAAAIbGVhc2VfaWQAAAAGAAAAAQAAA+kAAAfQAAAADEFjY2Vzc1N0YXR1cwAAAAM=",
        "AAAAAAAAAAAAAAAKaW5pdGlhbGl6ZQAAAAAAAQAAAAAAAAAFYWRtaW4AAAAAAAATAAAAAQAAA+kAAAPtAAAAAAAAAAM=",
        "AAAAAAAAAAAAAAALY3JlYXRlX3BsYW4AAAAACQAAAAAAAAAIb3BlcmF0b3IAAAATAAAAAAAAAAdwbGFuX2lkAAAAAAQAAAAAAAAABG5hbWUAAAAQAAAAAAAAAAV0b2tlbgAAAAAAABMAAAAAAAAACmRhaWx5X3JhdGUAAAAAAAsAAAAAAAAAC3RvdGFsX3ByaWNlAAAAAAsAAAAAAAAADmRlcG9zaXRfYW1vdW50AAAAAAALAAAAAAAAAAttaW5fcGF5bWVudAAAAAALAAAAAAAAABRncmFjZV9wZXJpb2Rfc2Vjb25kcwAAAAYAAAABAAAD6QAAA+0AAAAAAAAAAw==",
        "AAAAAAAAAAAAAAALY3JlYXRlX3Bvb2wAAAAABgAAAAAAAAAIb3BlcmF0b3IAAAATAAAAAAAAAAdwb29sX2lkAAAAAAYAAAAAAAAABXRva2VuAAAAAAAAEwAAAAAAAAAEbmFtZQAAABAAAAAAAAAADXRhcmdldF9hbW91bnQAAAAAAAALAAAAAAAAAA1yZXBheW1lbnRfYnBzAAAAAAAABAAAAAEAAAPpAAAD7QAAAAAAAAAD",
        "AAAAAAAAAAAAAAALZGV2aWNlX3N3YXAAAAAABAAAAAAAAAAIb3BlcmF0b3IAAAATAAAAAAAAAAhsZWFzZV9pZAAAAAYAAAAAAAAADW5ld19kZXZpY2VfaWQAAAAAAAPuAAAAIAAAAAAAAAAGcmVhc29uAAAAAAAQAAAAAQAAA+kAAAPtAAAAAAAAAAM=",
        "AAAAAAAAAAAAAAAMY3JlYXRlX2xlYXNlAAAABgAAAAAAAAAIb3BlcmF0b3IAAAATAAAAAAAAAAhsZWFzZV9pZAAAAAYAAAAAAAAACGN1c3RvbWVyAAAAEwAAAAAAAAAHcGxhbl9pZAAAAAAEAAAAAAAAAAlkZXZpY2VfaWQAAAAAAAPuAAAAIAAAAAAAAAAHcG9vbF9pZAAAAAPoAAAABgAAAAEAAAPpAAAD7QAAAAAAAAAD",
        "AAAAAAAAAAAAAAAMZ2V0X29wZXJhdG9yAAAAAQAAAAAAAAAIb3BlcmF0b3IAAAATAAAAAQAAA+kAAAfQAAAACE9wZXJhdG9yAAAAAw==",
        "AAAAAAAAAAAAAAAMZ3JhbnRfY3JlZGl0AAAABAAAAAAAAAAIb3BlcmF0b3IAAAATAAAAAAAAAAhsZWFzZV9pZAAAAAYAAAAAAAAABGRheXMAAAAEAAAAAAAAAAZyZWFzb24AAAAAABAAAAABAAAD6QAAAAYAAAAD",
        "AAAAAAAAAAAAAAANc2V0X3N1c3BlbmRlZAAAAAAAAAQAAAAAAAAACG9wZXJhdG9yAAAAEwAAAAAAAAAIbGVhc2VfaWQAAAAGAAAAAAAAAAlzdXNwZW5kZWQAAAAAAAABAAAAAAAAAAZyZWFzb24AAAAAABAAAAABAAAD6QAAA+0AAAAAAAAAAw==",
        "AAAAAAAAAAAAAAAPZW1lcmdlbmN5X3BhdXNlAAAAAAQAAAAAAAAACG9wZXJhdG9yAAAAEwAAAAAAAAAIbGVhc2VfaWQAAAAGAAAAAAAAABZwYXVzZV9kdXJhdGlvbl9zZWNvbmRzAAAAAAAGAAAAAAAAAAZyZWFzb24AAAAAABAAAAABAAAD6QAAAAYAAAAD",
        "AAAAAAAAAAAAAAAPc2V0X3BsYW5fYWN0aXZlAAAAAAMAAAAAAAAACG9wZXJhdG9yAAAAEwAAAAAAAAAHcGxhbl9pZAAAAAAEAAAAAAAAAAZhY3RpdmUAAAAAAAEAAAABAAAD6QAAA+0AAAAAAAAAAw==",
        "AAAAAAAAAAAAAAAQZ2V0X2RldmljZV9sZWFzZQAAAAEAAAAAAAAACWRldmljZV9pZAAAAAAAA+4AAAAgAAAAAQAAA+kAAAAGAAAAAw==",
        "AAAAAAAAAAAAAAARcmVnaXN0ZXJfb3BlcmF0b3IAAAAAAAADAAAAAAAAAAhvcGVyYXRvcgAAABMAAAAAAAAABG5hbWUAAAAQAAAAAAAAAA5wYXlvdXRfYWRkcmVzcwAAAAAAEwAAAAEAAAPpAAAD7QAAAAAAAAAD",
        "AAAAAAAAAAAAAAASYWNjZXB0X3BsYW5fY2hhbmdlAAAAAAACAAAAAAAAAAhjdXN0b21lcgAAABMAAAAAAAAACGxlYXNlX2lkAAAABgAAAAEAAAPpAAAD7QAAAAAAAAAD",
        "AAAAAAAAAAAAAAASZ2V0X3Bvb2xfZmluYW5jaWVyAAAAAAACAAAAAAAAAAdwb29sX2lkAAAAAAYAAAAAAAAACWZpbmFuY2llcgAAAAAAABMAAAABAAAD6QAAB9AAAAANUG9vbEZpbmFuY2llcgAAAAAAAAM=",
        "AAAAAAAAAAAAAAASaXNfZGV2aWNlX2Fzc2lnbmVkAAAAAAABAAAAAAAAAAlkZXZpY2VfaWQAAAAAAAPuAAAAIAAAAAEAAAAB",
        "AAAAAAAAAAAAAAATYmF0Y2hfY3JlYXRlX2xlYXNlcwAAAAACAAAAAAAAAAhvcGVyYXRvcgAAABMAAAAAAAAABmxlYXNlcwAAAAAD6gAAB9AAAAAPQmF0Y2hMZWFzZUlucHV0AAAAAAEAAAPpAAAABAAAAAM=",
        "AAAAAAAAAAAAAAATY2xhaW1fcG9vbF9lYXJuaW5ncwAAAAACAAAAAAAAAAlmaW5hbmNpZXIAAAAAAAATAAAAAAAAAAdwb29sX2lkAAAAAAYAAAABAAAD6QAAAAsAAAAD",
        "AAAAAAAAAAAAAAATcmVxdWVzdF9wbGFuX2NoYW5nZQAAAAADAAAAAAAAAAhvcGVyYXRvcgAAABMAAAAAAAAACGxlYXNlX2lkAAAABgAAAAAAAAALbmV3X3BsYW5faWQAAAAABAAAAAEAAAPpAAAD7QAAAAAAAAAD",
        "AAAAAAAAAAAAAAATc2V0X29wZXJhdG9yX2FjdGl2ZQAAAAACAAAAAAAAAAhvcGVyYXRvcgAAABMAAAAAAAAABmFjdGl2ZQAAAAAAAQAAAAEAAAPpAAAD7QAAAAAAAAAD",
        "AAAAAAAAAAAAAAAWdXBkYXRlX29wZXJhdG9yX3BheW91dAAAAAAAAgAAAAAAAAAIb3BlcmF0b3IAAAATAAAAAAAAAApuZXdfcGF5b3V0AAAAAAATAAAAAQAAA+kAAAPtAAAAAAAAAAM=",
        "AAAAAAAAAAAAAAAXZ2V0X3BlbmRpbmdfcGxhbl9jaGFuZ2UAAAAAAQAAAAAAAAAIbGVhc2VfaWQAAAAGAAAAAQAAA+kAAAAEAAAAAw==",
        "AAAAAAAAAAAAAAAXdHJhbnNmZXJfbGVhc2VfY3VzdG9tZXIAAAAAAwAAAAAAAAAQY3VycmVudF9jdXN0b21lcgAAABMAAAAAAAAADG5ld19jdXN0b21lcgAAABMAAAAAAAAACGxlYXNlX2lkAAAABgAAAAEAAAPpAAAD7QAAAAAAAAAD",
        "AAAAAAAAAAAAAAAYY2FsY3VsYXRlX2FjY2Vzc19zZWNvbmRzAAAAAgAAAAAAAAAGYW1vdW50AAAAAAALAAAAAAAAAApkYWlseV9yYXRlAAAAAAALAAAAAQAAAAY=",
        "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",
        "AAAAAQAAAAAAAAAAAAAADVBvb2xGaW5hbmNpZXIAAAAAAAAFAAAAAAAAAA5jbGFpbWVkX2Ftb3VudAAAAAAACwAAAAAAAAAOZGVwb3NpdF9hbW91bnQAAAAAAAsAAAAAAAAACWZpbmFuY2llcgAAAAAAABMAAAAAAAAAB3Bvb2xfaWQAAAAABgAAAAAAAAALcmV3YXJkX2RlYnQAAAAACw==",
        "AAAAAQAAAAAAAAAAAAAAD0JhdGNoTGVhc2VJbnB1dAAAAAAFAAAAAAAAAAhjdXN0b21lcgAAABMAAAAAAAAACWRldmljZV9pZAAAAAAAA+4AAAAgAAAAAAAAAAhsZWFzZV9pZAAAAAYAAAAAAAAAB3BsYW5faWQAAAAABAAAAAAAAAAHcG9vbF9pZAAAAAPoAAAABg==",
        "AAAABAAAAAAAAAAAAAAABUVycm9yAAAAAAAAIwAAAAAAAAASQWxyZWFkeUluaXRpYWxpemVkAAAAAAABAAAAAAAAAA5Ob3RJbml0aWFsaXplZAAAAAAAAgAAAAAAAAAMVW5hdXRob3JpemVkAAAAAwAAAAAAAAAQT3BlcmF0b3JOb3RGb3VuZAAAAAQAAAAAAAAAFU9wZXJhdG9yQWxyZWFkeUV4aXN0cwAAAAAAAAUAAAAAAAAAEE9wZXJhdG9ySW5hY3RpdmUAAAAGAAAAAAAAAAxQbGFuTm90Rm91bmQAAAAHAAAAAAAAABFQbGFuQWxyZWFkeUV4aXN0cwAAAAAAAAgAAAAAAAAADFBsYW5JbmFjdGl2ZQAAAAkAAAAAAAAADUxlYXNlTm90Rm91bmQAAAAAAAAKAAAAAAAAABJMZWFzZUFscmVhZHlFeGlzdHMAAAAAAAsAAAAAAAAADkxlYXNlTm90QWN0aXZlAAAAAAAMAAAAAAAAABFMZWFzZUFscmVhZHlPd25lZAAAAAAAAA0AAAAAAAAADkxlYXNlU3VzcGVuZGVkAAAAAAAOAAAAAAAAABBMZWFzZVJlcG9zc2Vzc2VkAAAADwAAAAAAAAANSW52YWxpZEFtb3VudAAAAAAAABAAAAAAAAAAE0JlbG93TWluaW11bVBheW1lbnQAAAAAEQAAAAAAAAAZT3ZlcnBheW1lbnRFeGNlZWRzQmFsYW5jZQAAAAAAABIAAAAAAAAAFlJlcG9zc2Vzc2lvbk5vdEFsbG93ZWQAAAAAABMAAAAAAAAAEUdyYWNlUGVyaW9kQWN0aXZlAAAAAAAAFAAAAAAAAAAMUG9vbE5vdEZvdW5kAAAAFQAAAAAAAAARUG9vbEFscmVhZHlFeGlzdHMAAAAAAAAWAAAAAAAAAA9Qb29sRnVsbHlGdW5kZWQAAAAAFwAAAAAAAAAZUG9vbEZ1bmRpbmdUYXJnZXRFeGNlZWRlZAAAAAAAABgAAAAAAAAAEU5vRWFybmluZ3NUb0NsYWltAAAAAAAAGQAAAAAAAAAVRGV2aWNlQWxyZWFkeUFzc2lnbmVkAAAAAAAAGgAAAAAAAAARSW52YWxpZFBsYW5QYXJhbXMAAAAAAAAbAAAAAAAAABJJbnZhbGlkQmFzaXNQb2ludHMAAAAAABwAAAAAAAAAEVBsYW5DaGFuZ2VQZW5kaW5nAAAAAAAAHQAAAAAAAAATTm9QZW5kaW5nUGxhbkNoYW5nZQAAAAAeAAAAAAAAABJQbGFuQ2hhbmdlTWlzbWF0Y2gAAAAAAB8AAAAAAAAAEUJhdGNoU2l6ZUV4Y2VlZGVkAAAAAAAAIAAAAAAAAAASQXJpdGhtZXRpY092ZXJmbG93AAAAAAAhAAAAAAAAAAtaZXJvQWRkcmVzcwAAAAAiAAAAAAAAABRQYXVzZUR1cmF0aW9uSW52YWxpZAAAACM=" ]),
      options
    )
  }
  public readonly fromJSON = {
    pay: this.txFromJSON<Result<i128>>,
        version: this.txFromJSON<u32>,
        get_plan: this.txFromJSON<Result<Plan>>,
        get_pool: this.txFromJSON<Result<FinancierPool>>,
        fund_pool: this.txFromJSON<Result<void>>,
        get_admin: this.txFromJSON<Result<string>>,
        get_lease: this.txFromJSON<Result<Lease>>,
        is_active: this.txFromJSON<boolean>,
        repossess: this.txFromJSON<Result<void>>,
        set_admin: this.txFromJSON<Result<void>>,
        get_access: this.txFromJSON<Result<AccessStatus>>,
        initialize: this.txFromJSON<Result<void>>,
        create_plan: this.txFromJSON<Result<void>>,
        create_pool: this.txFromJSON<Result<void>>,
        device_swap: this.txFromJSON<Result<void>>,
        create_lease: this.txFromJSON<Result<void>>,
        get_operator: this.txFromJSON<Result<Operator>>,
        grant_credit: this.txFromJSON<Result<u64>>,
        set_suspended: this.txFromJSON<Result<void>>,
        emergency_pause: this.txFromJSON<Result<u64>>,
        set_plan_active: this.txFromJSON<Result<void>>,
        get_device_lease: this.txFromJSON<Result<u64>>,
        register_operator: this.txFromJSON<Result<void>>,
        accept_plan_change: this.txFromJSON<Result<void>>,
        get_pool_financier: this.txFromJSON<Result<PoolFinancier>>,
        is_device_assigned: this.txFromJSON<boolean>,
        batch_create_leases: this.txFromJSON<Result<u32>>,
        claim_pool_earnings: this.txFromJSON<Result<i128>>,
        request_plan_change: this.txFromJSON<Result<void>>,
        set_operator_active: this.txFromJSON<Result<void>>,
        update_operator_payout: this.txFromJSON<Result<void>>,
        get_pending_plan_change: this.txFromJSON<Result<u32>>,
        transfer_lease_customer: this.txFromJSON<Result<void>>,
        calculate_access_seconds: this.txFromJSON<u64>,
        get_claimable_pool_earnings: this.txFromJSON<Result<i128>>
  }
}
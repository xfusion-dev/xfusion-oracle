import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface Bar {
  'low' : bigint,
  'high' : bigint,
  'close' : bigint,
  'open' : bigint,
  'volume' : [] | [bigint],
  'timestamp' : bigint,
}
export interface OracleMetrics {
  'total_updates' : bigint,
  'last_update_time' : bigint,
  'version' : bigint,
  'total_symbols' : bigint,
  'canister_cycles' : bigint,
}
export interface Policy { 'aggregation' : string, 'retain_history' : number }
export interface Price {
  'value' : bigint,
  'source' : string,
  'timestamp' : bigint,
  'confidence' : [] | [bigint],
}
export interface PriceUpdate { 'price' : Price, 'symbol' : Symbol }
export type Symbol = string;
export interface _SERVICE {
  'get_all_symbols' : ActorMethod<[], Array<Symbol>>,
  'get_available_resolutions' : ActorMethod<[], Array<string>>,
  'get_managers' : ActorMethod<[], Array<string>>,
  'get_metrics' : ActorMethod<[], OracleMetrics>,
  'get_price' : ActorMethod<[Symbol], [] | [Price]>,
  'get_price_history' : ActorMethod<[Symbol], Array<Price>>,
  'get_price_history_count' : ActorMethod<[Symbol], bigint>,
  'get_prices' : ActorMethod<[Array<Symbol>], Array<[] | [Price]>>,
  'get_range' : ActorMethod<[Symbol, bigint, bigint, string], Array<Bar>>,
  'get_snapshot_cert' : ActorMethod<
    [],
    [Array<[Symbol, Price]>, Uint8Array | number[]]
  >,
  'get_updaters' : ActorMethod<[], Array<string>>,
  'push_prices' : ActorMethod<[Array<PriceUpdate>], bigint>,
  'remove_symbols' : ActorMethod<[Array<Symbol>], undefined>,
  'set_allowed_updaters' : ActorMethod<[Array<Principal>], undefined>,
  'set_managers' : ActorMethod<[Array<Principal>], undefined>,
  'set_policy' : ActorMethod<[Policy], undefined>,
  'upsert_symbols' : ActorMethod<[Array<Symbol>], undefined>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];

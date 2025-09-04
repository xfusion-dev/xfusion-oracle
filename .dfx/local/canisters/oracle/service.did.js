export const idlFactory = ({ IDL }) => {
  const Symbol = IDL.Text;
  const OracleMetrics = IDL.Record({
    'total_updates' : IDL.Nat64,
    'last_update_time' : IDL.Nat64,
    'version' : IDL.Nat64,
    'total_symbols' : IDL.Nat64,
    'canister_cycles' : IDL.Nat64,
  });
  const Price = IDL.Record({
    'value' : IDL.Nat64,
    'source' : IDL.Text,
    'timestamp' : IDL.Nat64,
    'confidence' : IDL.Opt(IDL.Nat64),
  });
  const Bar = IDL.Record({
    'low' : IDL.Nat64,
    'high' : IDL.Nat64,
    'close' : IDL.Nat64,
    'open' : IDL.Nat64,
    'volume' : IDL.Opt(IDL.Nat64),
    'timestamp' : IDL.Nat64,
  });
  const PriceUpdate = IDL.Record({ 'price' : Price, 'symbol' : Symbol });
  const Policy = IDL.Record({
    'aggregation' : IDL.Text,
    'retain_history' : IDL.Nat32,
  });
  return IDL.Service({
    'get_all_symbols' : IDL.Func([], [IDL.Vec(Symbol)], ['query']),
    'get_available_resolutions' : IDL.Func([], [IDL.Vec(IDL.Text)], ['query']),
    'get_managers' : IDL.Func([], [IDL.Vec(IDL.Text)], ['query']),
    'get_metrics' : IDL.Func([], [OracleMetrics], ['query']),
    'get_price' : IDL.Func([Symbol], [IDL.Opt(Price)], ['query']),
    'get_price_history' : IDL.Func([Symbol], [IDL.Vec(Price)], ['query']),
    'get_price_history_count' : IDL.Func([Symbol], [IDL.Nat64], ['query']),
    'get_prices' : IDL.Func(
        [IDL.Vec(Symbol)],
        [IDL.Vec(IDL.Opt(Price))],
        ['query'],
      ),
    'get_range' : IDL.Func(
        [Symbol, IDL.Nat64, IDL.Nat64, IDL.Text],
        [IDL.Vec(Bar)],
        ['query'],
      ),
    'get_snapshot_cert' : IDL.Func(
        [],
        [IDL.Vec(IDL.Tuple(Symbol, Price)), IDL.Vec(IDL.Nat8)],
        ['query'],
      ),
    'get_updaters' : IDL.Func([], [IDL.Vec(IDL.Text)], ['query']),
    'push_prices' : IDL.Func([IDL.Vec(PriceUpdate)], [IDL.Nat64], []),
    'remove_symbols' : IDL.Func([IDL.Vec(Symbol)], [], []),
    'set_allowed_updaters' : IDL.Func([IDL.Vec(IDL.Principal)], [], []),
    'set_managers' : IDL.Func([IDL.Vec(IDL.Principal)], [], []),
    'set_policy' : IDL.Func([Policy], [], []),
    'upsert_symbols' : IDL.Func([IDL.Vec(Symbol)], [], []),
  });
};
export const init = ({ IDL }) => { return []; };

import { Actor, HttpAgent } from '@dfinity/agent';

// Define the Oracle service interface based on the DID file
export interface Price {
  value: bigint;
  confidence: [] | [bigint];
  timestamp: bigint;
  source: string;
}

export interface OracleMetrics {
  total_symbols: bigint;
  total_updates: bigint;
  last_update_time: bigint;
  canister_cycles: bigint;
  version: bigint;
}

// Types are already exported above with export interface

// Oracle service interface
export interface OracleService {
  get_price: (symbol: string) => Promise<[] | [Price]>;
  get_prices: (symbols: string[]) => Promise<([] | [Price])[]>;
  get_all_symbols: () => Promise<string[]>;
  get_metrics: () => Promise<OracleMetrics>;
  get_managers: () => Promise<string[]>;
  get_updaters: () => Promise<string[]>;
}

// IDL factory for the oracle canister
const idlFactory = ({ IDL }: any) => {
  const Price = IDL.Record({
    value: IDL.Nat64,
    confidence: IDL.Opt(IDL.Nat64),
    timestamp: IDL.Nat64,
    source: IDL.Text,
  });

  const OracleMetrics = IDL.Record({
    total_symbols: IDL.Nat64,
    total_updates: IDL.Nat64,
    last_update_time: IDL.Nat64,
    canister_cycles: IDL.Nat64,
    version: IDL.Nat64,
  });

  return IDL.Service({
    get_price: IDL.Func([IDL.Text], [IDL.Opt(Price)], ['query']),
    get_prices: IDL.Func([IDL.Vec(IDL.Text)], [IDL.Vec(IDL.Opt(Price))], ['query']),
    get_all_symbols: IDL.Func([], [IDL.Vec(IDL.Text)], ['query']),
    get_metrics: IDL.Func([], [OracleMetrics], ['query']),
    get_managers: IDL.Func([], [IDL.Vec(IDL.Text)], ['query']),
    get_updaters: IDL.Func([], [IDL.Vec(IDL.Text)], ['query']),
  });
};

// Oracle client class
export class OracleClient {
  private actor: OracleService;

  constructor(canisterId: string, options?: { host?: string }) {
    const agent = new HttpAgent({
      host: options?.host || 'http://127.0.0.1:4943',
    });

    // Fetch root key for local development
    if (options?.host?.includes('127.0.0.1') || !options?.host) {
      agent.fetchRootKey().catch(console.error);
    }

    this.actor = Actor.createActor(idlFactory, {
      agent,
      canisterId,
    }) as OracleService;
  }

  async getPrice(symbol: string): Promise<Price | null> {
    const result = await this.actor.get_price(symbol);
    return result.length > 0 ? result[0] : null;
  }

  async getPrices(symbols: string[]): Promise<(Price | null)[]> {
    const results = await this.actor.get_prices(symbols);
    return results.map(result => result.length > 0 ? result[0] : null);
  }

  async getAllSymbols(): Promise<string[]> {
    return await this.actor.get_all_symbols();
  }

  async getMetrics(): Promise<OracleMetrics> {
    return await this.actor.get_metrics();
  }

  async getManagers(): Promise<string[]> {
    return await this.actor.get_managers();
  }

  async getUpdaters(): Promise<string[]> {
    return await this.actor.get_updaters();
  }
}

// Utility functions
export const formatPrice = (value: bigint): string => {
  // Convert from 8-decimal precision to readable price
  const price = Number(value) / 100_000_000;
  return new Intl.NumberFormat('en-US', {
    style: 'currency',
    currency: 'USD',
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  }).format(price);
};

export const formatTimestamp = (timestamp: bigint): string => {
  const date = new Date(Number(timestamp) / 1_000_000); // Convert nanoseconds to milliseconds
  return date.toLocaleString();
};

export const formatTimeAgo = (timestamp: bigint): string => {
  const now = Date.now();
  const updateTime = Number(timestamp) / 1_000_000; // Convert nanoseconds to milliseconds
  const diffSeconds = Math.floor((now - updateTime) / 1000);
  
  if (diffSeconds < 60) {
    return `${diffSeconds} seconds ago`;
  } else if (diffSeconds < 3600) {
    const minutes = Math.floor(diffSeconds / 60);
    return `${minutes} minute${minutes === 1 ? '' : 's'} ago`;
  } else {
    const hours = Math.floor(diffSeconds / 3600);
    return `${hours} hour${hours === 1 ? '' : 's'} ago`;
  }
};

export const formatConfidence = (confidence: [] | [bigint]): string => {
  if (confidence.length === 0) return 'N/A';
  const conf = Number(confidence[0]) / 100_000_000;
  return `±$${conf.toFixed(2)}`;
};

// Create oracle client instance - use mainnet canister ID
export const oracleClient = new OracleClient('zutfo-jqaaa-aaaao-a4puq-cai', {
  host: 'https://ic0.app'
});

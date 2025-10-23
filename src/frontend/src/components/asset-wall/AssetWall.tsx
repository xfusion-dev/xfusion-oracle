import { useMemo, useRef, useEffect, useState, memo } from 'react';
import { motion } from 'framer-motion';
import { TrendingUp, TrendingDown } from 'lucide-react';
import { useOraclePrices, useOracleMetrics } from '../../hooks/useOracleData';
import { formatPrice, formatTimeAgo } from '../../services/oracle';

interface WallAsset {
  symbol: string;
  name: string;
  price: number | null;
  change24h: number | null;
}

function toDisplayName(symbol: string): string {
  const mapping: Record<string, string> = {
    BTC: 'Bitcoin', ETH: 'Ethereum', ICP: 'Internet Computer', SOL: 'Solana',
    USDT: 'Tether', USDC: 'USD Coin', BNB: 'BNB', XRP: 'Ripple', ADA: 'Cardano',
    AVAX: 'Avalanche', DOGE: 'Dogecoin', DOT: 'Polkadot', LINK: 'Chainlink',
    MATIC: 'Polygon', UNI: 'Uniswap', ATOM: 'Cosmos', LTC: 'Litecoin', BCH: 'Bitcoin Cash',
    NEAR: 'NEAR Protocol', APT: 'Aptos', FIL: 'Filecoin', ARB: 'Arbitrum', OP: 'Optimism',
    INJ: 'Injective', TIA: 'Celestia', STX: 'Stacks', RUNE: 'THORChain', AAVE: 'Aave',
    SNX: 'Synthetix', WAL: 'Wallet'
  };
  return mapping[symbol] || symbol;
}

// CoinMarketCap ID mapping for icon URLs
function getCMCId(symbol: string): number | null {
  const mapping: Record<string, number> = {
    BTC: 1, ETH: 1027, ICP: 8916, SOL: 5426, USDT: 825, USDC: 3408,
    BNB: 1839, XRP: 52, ADA: 2010, AVAX: 5805, DOGE: 74, DOT: 6636,
    LINK: 1975, MATIC: 3890, UNI: 7083, ATOM: 3794, LTC: 2, BCH: 1831,
    NEAR: 6535, APT: 21794, FIL: 2280, ARB: 11841, OP: 11840, INJ: 7226,
    TIA: 22861, STX: 4847, RUNE: 4157, AAVE: 7278, SNX: 2586
  };
  return mapping[symbol] || null;
}

// Memoized AssetCard component - only re-renders when its props change
const AssetCard = memo(function AssetCard({ 
  symbol, 
  name, 
  price, 
  prevPrice 
}: { 
  symbol: string; 
  name: string; 
  price: number | null; 
  prevPrice: number | null;
}) {
  const [imgError, setImgError] = useState(false);
  const isUp = price != null && prevPrice != null && price > prevPrice;
  const isDown = price != null && prevPrice != null && price < prevPrice;
  const cmcId = getCMCId(symbol);

  return (
    <motion.div 
      animate={{ 
        scale: isUp || isDown ? 1.02 : 1, 
        backgroundColor: (isUp || isDown) ? 'rgba(255,255,255,0.08)' : 'rgba(255,255,255,0.05)' 
      }} 
      transition={{ duration: 0.25 }} 
      className="border border-white/10 p-3 hover:bg-white/10 transition-colors"
    >
      <div className="flex items-start justify-between mb-2">
        <div className="flex items-center gap-2 flex-1 min-w-0">
          {/* Icon */}
          {cmcId && !imgError ? (
            <img 
              src={`https://s2.coinmarketcap.com/static/img/coins/64x64/${cmcId}.png`}
              alt={symbol}
              className="w-6 h-6 rounded-full flex-shrink-0"
              onError={() => setImgError(true)}
            />
          ) : (
            <div className="w-6 h-6 bg-primary/20 rounded-full flex items-center justify-center flex-shrink-0">
              <span className="text-primary font-mono text-[10px] font-bold">{symbol[0]}</span>
            </div>
          )}
          
          <div className="flex-1 min-w-0">
            <div className="text-white font-bold text-sm truncate">{symbol}</div>
            <div className="text-[10px] text-gray-500 truncate">{name}</div>
          </div>
        </div>
        
        {/* Trend indicator */}
        {isUp ? (
          <TrendingUp className="w-3 h-3 text-green-400 flex-shrink-0 ml-1" />
        ) : isDown ? (
          <TrendingDown className="w-3 h-3 text-red-400 flex-shrink-0 ml-1" />
        ) : (
          <div className="w-3 h-3" />
        )}
      </div>

      <div className="space-y-0.5">
        <div className="text-white font-mono text-xs">
          {price == null ? 'No data' : `$${price < 10 ? price.toFixed(4) : price.toFixed(2)}`}
        </div>
        <div className="text-[10px] font-mono text-gray-400">
          {price == null ? '' : formatPrice(BigInt(Math.round(price * 100_000_000)))}
        </div>
      </div>
    </motion.div>
  );
});

// Time ago component that updates every second
function LastUpdateTime({ timestamp }: { timestamp: bigint | null }) {
  const [timeAgo, setTimeAgo] = useState('');

  useEffect(() => {
    if (!timestamp || timestamp === 0n) {
      setTimeAgo('N/A');
      return;
    }

    const updateTimeAgo = () => {
      setTimeAgo(formatTimeAgo(timestamp));
    };

    updateTimeAgo();
    const interval = setInterval(updateTimeAgo, 1000);
    return () => clearInterval(interval);
  }, [timestamp]);

  return <>{timeAgo}</>;
}

export function AssetWall() {
  const { data: assetsRaw } = useOraclePrices();
  const lastValuesRef = useRef<Record<string, number>>({});

  const assets: WallAsset[] = useMemo(() => {
    return (assetsRaw || []).map((a) => {
      const priceNum = a.price ? Number(a.price.value) / 100_000_000 : null;
      return {
        symbol: a.symbol,
        name: toDisplayName(a.symbol),
        price: priceNum,
        change24h: null,
      };
    });
  }, [assetsRaw]);

  // Update lastValues ref on price changes (doesn't trigger re-render)
  useEffect(() => {
    const current: Record<string, number> = {};
    for (const a of assets) {
      if (a.price != null) {
        current[a.symbol] = a.price;
      }
    }
    lastValuesRef.current = current;
  }, [assets]);

  // Get metrics for stats
  const { data: metrics } = useOracleMetrics();

  return (
    <section className="relative bg-black overflow-hidden py-16 md:py-24">
      <div className="absolute inset-0 opacity-[0.03]" style={{ backgroundImage: `linear-gradient(white 1px, transparent 1px), linear-gradient(90deg, white 1px, transparent 1px)`, backgroundSize: '50px 50px' }} />
      <div className="relative z-10 px-6">
        <div className="w-full max-w-7xl mx-auto">
          <motion.div initial={{ opacity: 0, y: 30 }} animate={{ opacity: 1, y: 0 }} transition={{ duration: 0.6 }} className="mb-8">
            <h2 className="text-3xl md:text-5xl font-bold text-white mb-2">Live Asset Prices</h2>
            <p className="text-gray-400 max-w-2xl">Track real-time prices from the oracle canister</p>
          </motion.div>

          {/* Removed key={touch} - grid no longer re-mounts on updates */}
          <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 gap-2">
            {assets.map((asset) => (
              <AssetCard 
                key={asset.symbol}
                symbol={asset.symbol}
                name={asset.name}
                price={asset.price}
                prevPrice={lastValuesRef.current[asset.symbol] ?? null}
              />
            ))}
          </div>

          <motion.div initial={{ opacity: 0, y: 10 }} animate={{ opacity: 1, y: 0 }} transition={{ duration: 0.4, delay: 0.1 }} className="mt-8 bg-white/5 border border-white/10 p-4">
            <div className="grid grid-cols-2 md:grid-cols-3 gap-4">
              <div>
                <div className="text-xs text-gray-500 mb-1">Total Assets</div>
                <div className="text-2xl font-bold text-white">{assets.length}</div>
              </div>
              <div>
                <div className="text-xs text-gray-500 mb-1">Total Updates</div>
                <div className="text-2xl font-bold text-white">{metrics ? Number(metrics.total_updates).toLocaleString() : '...'}</div>
              </div>
              <div>
                <div className="text-xs text-gray-500 mb-1">Last Update</div>
                <div className="text-2xl font-bold text-white">
                  <LastUpdateTime timestamp={metrics?.last_update_time ?? null} />
                </div>
              </div>
            </div>
          </motion.div>
        </div>
      </div>
    </section>
  );
}

export default AssetWall;



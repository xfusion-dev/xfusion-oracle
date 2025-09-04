// Mock data for price feeds
const mockPriceFeeds = [
  { symbol: 'BTC', pair: 'BTC/USD', price: '67,533.42', change: '+2.3%', isPositive: true, sparkline: [65000, 66200, 67000, 66800, 67533] },
  { symbol: 'ETH', pair: 'ETH/USD', price: '2,847.91', change: '+1.8%', isPositive: true, sparkline: [2800, 2820, 2850, 2840, 2847] },
  { symbol: 'SOL', pair: 'SOL/USD', price: '142.67', change: '-0.5%', isPositive: false, sparkline: [145, 144, 143, 142, 142.67] },
  { symbol: 'AVAX', pair: 'AVAX/USD', price: '28.43', change: '+4.2%', isPositive: true, sparkline: [27, 27.5, 28, 28.2, 28.43] },
  { symbol: 'MATIC', pair: 'MATIC/USD', price: '0.8921', change: '+1.1%', isPositive: true, sparkline: [0.88, 0.885, 0.89, 0.891, 0.8921] },
  { symbol: 'ADA', pair: 'ADA/USD', price: '0.3456', change: '-2.1%', isPositive: false, sparkline: [0.35, 0.348, 0.346, 0.345, 0.3456] },
  { symbol: 'DOT', pair: 'DOT/USD', price: '4.23', change: '+0.8%', isPositive: true, sparkline: [4.18, 4.19, 4.21, 4.22, 4.23] },
];

// Simple sparkline component
function Sparkline({ data, isPositive }: { data: number[]; isPositive: boolean }) {
  const max = Math.max(...data);
  const min = Math.min(...data);
  const range = max - min;
  
  const points = data.map((value, index) => {
    const x = (index / (data.length - 1)) * 100;
    const y = range === 0 ? 50 : ((max - value) / range) * 100;
    return `${x},${y}`;
  }).join(' ');

  return (
    <svg className="w-full h-12" viewBox="0 0 100 100" preserveAspectRatio="none">
      <polyline
        fill="none"
        stroke={isPositive ? "#10b981" : "#ef4444"}
        strokeWidth="2"
        points={points}
        className="opacity-80"
      />
    </svg>
  );
}

// Price feed card component
function PriceFeedCard({ symbol, pair, price, change, isPositive, sparkline }: {
  symbol: string;
  pair: string;
  price: string;
  change: string;
  isPositive: boolean;
  sparkline: number[];
}) {
  return (
    <div className="bg-elevated border border-primary/20 rounded-lg p-4 min-w-[280px] hover:border-primary/40 transition-all duration-300">
      <div className="flex items-center justify-between mb-3">
        <div className="flex items-center gap-2">
          <div className="w-8 h-8 bg-primary/20 rounded-full flex items-center justify-center">
            <span className="text-primary font-mono text-sm font-bold">{symbol[0]}</span>
          </div>
          <span className="text-white font-medium">{pair}</span>
        </div>
        <span className={`text-sm font-mono ${isPositive ? 'text-success-500' : 'text-error-500'}`}>
          {change}
        </span>
      </div>
      
      <div className="mb-3">
        <Sparkline data={sparkline} isPositive={isPositive} />
      </div>
      
      <div className="text-right">
        <span className="text-white text-xl font-bold font-mono">{price}</span>
      </div>
    </div>
  );
}

export default function HomePage() {
  return (
    <>
      {/* Hero Section - Refined and alive */}
      <section className="px-6 py-32 bg-void min-h-screen flex items-center justify-center relative overflow-hidden">
        {/* Dynamic background grid */}
        <div className="absolute inset-0 opacity-5">
          <div className="absolute inset-0" style={{
            backgroundImage: `
              linear-gradient(rgba(59, 130, 246, 0.1) 1px, transparent 1px),
              linear-gradient(90deg, rgba(59, 130, 246, 0.1) 1px, transparent 1px)
            `,
            backgroundSize: '50px 50px'
          }}></div>
        </div>

        {/* Floating data points */}
        <div className="absolute inset-0 overflow-hidden">
          <div className="absolute top-20 left-20 w-3 h-3 bg-primary/60 rounded-full animate-pulse shadow-lg shadow-primary/50"></div>
          <div className="absolute top-40 right-32 w-2 h-2 bg-success-500/60 rounded-full animate-ping shadow-lg shadow-success-500/50"></div>
          <div className="absolute bottom-40 left-1/3 w-4 h-4 bg-primary/40 rounded-full animate-pulse delay-1000 shadow-lg shadow-primary/30"></div>
          <div className="absolute top-1/3 right-20 w-2 h-2 bg-success-500/60 rounded-full animate-ping delay-500 shadow-lg shadow-success-500/50"></div>
          <div className="absolute bottom-20 right-1/4 w-3 h-3 bg-primary/60 rounded-full animate-pulse delay-2000 shadow-lg shadow-primary/50"></div>
          <div className="absolute top-60 left-1/2 w-1 h-1 bg-white/60 rounded-full animate-ping delay-1500"></div>
          <div className="absolute bottom-60 left-20 w-2 h-2 bg-unique/60 rounded-full animate-pulse delay-3000"></div>
        </div>

        {/* Radial gradient overlay */}
        <div className="absolute inset-0 bg-gradient-radial from-primary/5 via-transparent to-transparent"></div>

        <div className="max-w-7xl mx-auto text-center relative z-10">
          {/* Status indicator */}
          <div className="mb-8 inline-flex items-center gap-2">
            <div className="w-2 h-2 bg-success-500 animate-pulse"></div>
            <span className="text-primary text-sm font-mono uppercase tracking-wider">
              LIVE PRICES
            </span>
            <div className="w-2 h-2 bg-success-500 animate-pulse delay-500"></div>
          </div>

          {/* Main title with gradient */}
          <h1 className="text-7xl md:text-9xl font-bold mb-8 tracking-tight bg-gradient-to-r from-white via-white to-primary bg-clip-text text-transparent">
            ORACLES
          </h1>
          
          {/* Subtitle with better spacing */}
          <div className="mb-16">
            <p className="text-2xl md:text-3xl font-light text-white">
              Powered by XFusion.
            </p>
          </div>
          
          {/* Clean live stats */}
          <div className="grid grid-cols-3 gap-12 max-w-lg mx-auto mb-16">
            <div className="text-center">
              <div className="text-4xl font-bold text-white mb-2">30+</div>
              <div className="text-unique text-sm uppercase tracking-wider">Price Feeds</div>
            </div>
            <div className="text-center">
              <div className="text-4xl font-bold text-success-500 mb-2">15s</div>
              <div className="text-unique text-sm uppercase tracking-wider">Updates</div>
            </div>
            <div className="text-center">
              <div className="text-4xl font-bold text-primary mb-2">99.9%</div>
              <div className="text-unique text-sm uppercase tracking-wider">Uptime</div>
            </div>
          </div>

          {/* Clean CTAs */}
          <div className="flex flex-col sm:flex-row gap-6 justify-center">
            <button className="bg-primary hover:bg-primary-600 text-white px-10 py-4 font-medium transition-all duration-300 hover:shadow-lg hover:shadow-primary/30 hover:translate-y-[-2px]">
              Explore Feeds
            </button>
            <button className="bg-white hover:bg-gray-100 text-black px-10 py-4 font-medium transition-all duration-300 hover:shadow-lg hover:translate-y-[-2px]">
              View Documentation
            </button>
          </div>
        </div>
      </section>

      {/* Live Price Feeds Section - Sexy table */}
      <section className="px-6 py-24 bg-gradient-to-b from-void to-space relative">
        <div className="max-w-7xl mx-auto">
          <div className="text-center mb-16">
            <h2 className="text-4xl md:text-5xl font-bold text-white mb-4">
              Live Price Feeds
            </h2>
            <p className="text-unique text-lg max-w-2xl mx-auto">
              Real-time market data with sub-second latency and cryptographic verification
            </p>
          </div>

          {/* Sexy Price Table */}
          <div className="bg-elevated/50 backdrop-blur-xl border border-primary/20 rounded-2xl overflow-hidden shadow-2xl">
            <div className="overflow-x-auto">
              <table className="w-full">
                <thead>
                  <tr className="border-b border-primary/20 bg-primary/5">
                    <th className="text-left py-4 px-6 text-unique font-medium text-sm uppercase tracking-wider">Asset</th>
                    <th className="text-right py-4 px-6 text-unique font-medium text-sm uppercase tracking-wider">Price</th>
                    <th className="text-right py-4 px-6 text-unique font-medium text-sm uppercase tracking-wider">Last Update</th>
                  </tr>
                </thead>
                <tbody>
                  {mockPriceFeeds.map((feed, index) => (
                    <tr 
                      key={feed.symbol}
                      className="border-b border-primary/10 hover:bg-white transition-all duration-300 group"
                      style={{
                        animationDelay: `${index * 50}ms`,
                      }}
                    >
                      {/* Asset */}
                      <td className="py-4 px-6">
                        <div className="flex items-center gap-3">
                          <div className="w-10 h-10 bg-gradient-to-br from-primary to-primary-600 rounded-full flex items-center justify-center shadow-lg group-hover:scale-110 transition-transform duration-300">
                            <span className="text-white font-bold text-sm">{feed.symbol[0]}</span>
                          </div>
                          <div>
                            <div className="text-white group-hover:text-black font-semibold transition-colors duration-300">{feed.symbol}</div>
                            <div className="text-unique group-hover:text-gray-600 text-sm transition-colors duration-300">{feed.pair}</div>
                          </div>
                        </div>
                      </td>
                      
                      {/* Price */}
                      <td className="py-4 px-6 text-right">
                        <div className="text-white group-hover:text-black text-xl font-bold font-mono transition-colors duration-300">
                          ${feed.price}
                        </div>
                      </td>
                      
                      {/* Last Update */}
                      <td className="py-4 px-6 text-right">
                        <div className="text-unique group-hover:text-gray-600 text-sm transition-colors duration-300">
                          <span className="inline-block w-2 h-2 bg-success-500 rounded-full mr-2 animate-pulse"></span>
                          Live
                        </div>
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </div>

          {/* Bottom CTA */}
          <div className="text-center mt-16">
            <button className="border border-primary text-primary hover:bg-primary hover:text-white px-8 py-3 font-medium transition-all duration-300 hover:shadow-lg hover:shadow-primary/20 hover:translate-y-[-2px]">
              View All 30+ Price Feeds →
            </button>
          </div>
        </div>
      </section>
    </>
  );
}
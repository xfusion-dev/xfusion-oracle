import { Link } from 'react-router-dom';
import { useOracleMetrics } from '../hooks/useOracleData';
import { Suspense, lazy } from 'react';

const AssetWall = lazy(() => import('../components/asset-wall/AssetWall'));

export default function HomePage() {
  const { data: metrics } = useOracleMetrics();
  const isLoading = false;
  const error = false;

  if (isLoading) {
    return (
      <div className="px-6 py-24 bg-void min-h-screen flex items-center justify-center">
        <div className="text-white text-xl">Loading oracle data...</div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="px-6 py-24 bg-void min-h-screen flex items-center justify-center">
        <div className="text-error-500 text-xl">Error loading oracle data</div>
      </div>
    );
  }

  return (
    <>
      <section className="px-6 py-32 bg-black min-h-screen flex items-center justify-center relative overflow-hidden">
        <div className="absolute inset-0 opacity-[0.03]" style={{
          backgroundImage: `linear-gradient(white 1px, transparent 1px), linear-gradient(90deg, white 1px, transparent 1px)`,
          backgroundSize: '50px 50px'
        }} />

        <div className="max-w-7xl mx-auto text-center relative z-10">
          <div className="mb-8 inline-flex items-center gap-2">
            <div className="w-2 h-2 bg-success-500 animate-pulse"></div>
            <span className="text-primary text-sm font-mono uppercase tracking-wider">
              LIVE PRICES
            </span>
            <div className="w-2 h-2 bg-success-500 animate-pulse delay-500"></div>
          </div>

          <h1 className="text-7xl md:text-9xl font-bold mb-8 tracking-tight bg-gradient-to-r from-white via-white to-primary bg-clip-text text-transparent">
            ORACLES
          </h1>
          
          <div className="mb-16">
            <p className="text-2xl md:text-3xl font-light text-white">
              Powered by XFusion.
            </p>
          </div>
          
          <div className="grid grid-cols-3 gap-12 max-w-lg mx-auto mb-16">
            <div className="text-center">
              <div className="text-4xl font-bold text-white mb-2">{metrics ? Number(metrics.total_symbols) : '...'}</div>
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

          <div className="flex flex-col sm:flex-row gap-6 justify-center">
            <Link to="/assets"> 
              <button className="bg-primary hover:bg-primary-600 text-white px-10 py-4 font-medium transition-all duration-300 hover:shadow-lg hover:shadow-primary/30 hover:translate-y-[-2px]">
                Explore Feeds
              </button>
            </Link>
            <Link to="https://docs.xfusion.finance/oracle/getting-started" target="_blank" rel="noopener noreferrer">
              <button className="bg-white hover:bg-gray-100 text-black px-10 py-4 font-medium transition-all duration-300 hover:shadow-lg hover:translate-y-[-2px]">
                View Documentation
              </button>
            </Link>
          </div>
        </div>
      </section>

      <section>
        <Suspense fallback={<div className="py-24 text-center text-white">Loading...</div>}>
          <AssetWall />
        </Suspense>
      </section>
    </>
  );
}
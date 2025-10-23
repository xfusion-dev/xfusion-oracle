import { useOraclePrices, useOracleMetrics } from '../hooks/useOracleData';
import { formatPrice } from '../services/oracle';
import { useTimeAgo } from '../hooks/useTimeAgo';
import AssetWall from '../components/asset-wall/AssetWall';

function TimeAgoDisplay({ timestamp }: { timestamp: bigint | null }) {
  const timeAgo = useTimeAgo(timestamp);
  return <span>{timeAgo}</span>;
}

export default function AssetsPage() {
  const { data: allAssets, isLoading, error } = useOraclePrices();
  const { data: metrics } = useOracleMetrics();

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
    <div className="px-6 py-24 bg-void min-h-screen">
      <div className="max-w-7xl mx-auto">
        <div className="mb-16">
          <AssetWall />
        </div>
        <div className="mb-16">
          <h1 className="text-5xl md:text-6xl font-bold text-white mb-4">
            Assets
          </h1>
          <p className="text-unique text-lg max-w-2xl">
            Complete list of all supported price feeds with real-time data and source information.
          </p>
        </div>

      </div>
    </div>
  );
}

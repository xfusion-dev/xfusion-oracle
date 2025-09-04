import { allAssets } from '../data/mockAssets';

export default function AssetsPage() {
  return (
    <div className="px-6 py-24 bg-void min-h-screen">
      <div className="max-w-7xl mx-auto">
        {/* Page Header */}
        <div className="mb-16">
          <h1 className="text-5xl md:text-6xl font-bold text-white mb-4">
            Assets
          </h1>
          <p className="text-unique text-lg max-w-2xl">
            Complete list of all supported price feeds with real-time data and source information.
          </p>
        </div>

        {/* Assets Table */}
        <div className="bg-elevated/50 backdrop-blur-xl border border-primary/20 overflow-hidden shadow-2xl">
          <div className="overflow-x-auto">
            <table className="w-full">
              <thead>
                <tr className="border-b border-primary/20 bg-primary/5">
                  <th className="text-left py-4 px-6 text-unique font-medium text-sm uppercase tracking-wider">Asset</th>
                  <th className="text-right py-4 px-6 text-unique font-medium text-sm uppercase tracking-wider">Price</th>
                  <th className="text-left py-4 px-6 text-unique font-medium text-sm uppercase tracking-wider">Source</th>
                  <th className="text-right py-4 px-6 text-unique font-medium text-sm uppercase tracking-wider">Status</th>
                </tr>
              </thead>
              <tbody>
                {allAssets.map((asset, index) => (
                  <tr 
                    key={asset.symbol}
                    className="border-b border-primary/10 hover:bg-white transition-all duration-300 group"
                  >
                    {/* Asset */}
                    <td className="py-4 px-6">
                      <div className="flex items-center gap-3">
                        <div className="w-10 h-10 bg-gradient-to-br from-primary to-primary-600 flex items-center justify-center shadow-lg">
                          <span className="text-white font-bold text-sm">{asset.symbol[0]}</span>
                        </div>
                        <div>
                          <div className="text-white group-hover:text-black font-semibold transition-colors duration-300">{asset.symbol}</div>
                          <div className="text-unique group-hover:text-gray-600 text-sm transition-colors duration-300">{asset.pair}</div>
                        </div>
                      </div>
                    </td>
                    
                    {/* Price */}
                    <td className="py-4 px-6 text-right">
                      <div className="text-white group-hover:text-black text-xl font-bold font-mono transition-colors duration-300">
                        ${asset.price}
                      </div>
                    </td>
                    
                    {/* Source */}
                    <td className="py-4 px-6">
                      <div className="text-unique group-hover:text-gray-600 text-sm transition-colors duration-300">
                        {asset.source}
                      </div>
                    </td>
                    
                    {/* Status */}
                    <td className="py-4 px-6 text-right">
                      <div className="text-unique group-hover:text-gray-600 text-sm transition-colors duration-300">
                        <span className="inline-block w-2 h-2 bg-success-500 mr-2 animate-pulse"></span>
                        Active
                      </div>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>

        {/* Page Stats */}
        <div className="mt-16 text-center">
          <div className="grid grid-cols-1 md:grid-cols-3 gap-8 max-w-lg mx-auto">
            <div>
              <div className="text-2xl font-bold text-white mb-1">{allAssets.length}</div>
              <div className="text-unique text-sm">Total Assets</div>
            </div>
            <div>
              <div className="text-2xl font-bold text-success-500 mb-1">100%</div>
              <div className="text-unique text-sm">Active</div>
            </div>
            <div>
              <div className="text-2xl font-bold text-primary mb-1">24/7</div>
              <div className="text-unique text-sm">Monitoring</div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

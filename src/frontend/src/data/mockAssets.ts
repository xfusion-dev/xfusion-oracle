export const allAssets = [
  { symbol: 'BTC', pair: 'BTC/USD', price: '67,533.42', source: 'Pyth Network', change: '+2.3%', isPositive: true, sparkline: [65000, 66200, 67000, 66800, 67533] },
  { symbol: 'ETH', pair: 'ETH/USD', price: '2,847.91', source: 'Pyth Network', change: '+1.8%', isPositive: true, sparkline: [2800, 2820, 2850, 2840, 2847] },
  { symbol: 'SOL', pair: 'SOL/USD', price: '142.67', source: 'Pyth Network', change: '-0.5%', isPositive: false, sparkline: [145, 144, 143, 142, 142.67] },
  { symbol: 'AVAX', pair: 'AVAX/USD', price: '28.43', source: 'Pyth Network', change: '+4.2%', isPositive: true, sparkline: [27, 27.5, 28, 28.2, 28.43] },
  { symbol: 'MATIC', pair: 'MATIC/USD', price: '0.8921', source: 'Pyth Network', change: '+1.1%', isPositive: true, sparkline: [0.88, 0.885, 0.89, 0.891, 0.8921] },
  { symbol: 'ADA', pair: 'ADA/USD', price: '0.3456', source: 'Pyth Network', change: '-2.1%', isPositive: false, sparkline: [0.35, 0.348, 0.346, 0.345, 0.3456] },
  { symbol: 'DOT', pair: 'DOT/USD', price: '4.23', source: 'Pyth Network', change: '+0.8%', isPositive: true, sparkline: [4.18, 4.19, 4.21, 4.22, 4.23] },
  { symbol: 'LINK', pair: 'LINK/USD', price: '11.87', source: 'Pyth Network', change: '+1.5%', isPositive: true, sparkline: [11.7, 11.75, 11.8, 11.85, 11.87] },
  { symbol: 'UNI', pair: 'UNI/USD', price: '8.45', source: 'Pyth Network', change: '-0.3%', isPositive: false, sparkline: [8.5, 8.48, 8.46, 8.45, 8.45] },
  { symbol: 'AAVE', pair: 'AAVE/USD', price: '156.78', source: 'Pyth Network', change: '+3.2%', isPositive: true, sparkline: [152, 154, 155, 156, 156.78] },
  { symbol: 'ATOM', pair: 'ATOM/USD', price: '7.92', source: 'Pyth Network', change: '+0.9%', isPositive: true, sparkline: [7.85, 7.88, 7.9, 7.91, 7.92] },
  { symbol: 'FTM', pair: 'FTM/USD', price: '0.4123', source: 'Pyth Network', change: '-1.8%', isPositive: false, sparkline: [0.42, 0.418, 0.415, 0.413, 0.4123] },
  { symbol: 'NEAR', pair: 'NEAR/USD', price: '5.67', source: 'Pyth Network', change: '+2.1%', isPositive: true, sparkline: [5.55, 5.58, 5.62, 5.65, 5.67] },
  { symbol: 'ALGO', pair: 'ALGO/USD', price: '0.1876', source: 'Pyth Network', change: '-0.7%', isPositive: false, sparkline: [0.189, 0.188, 0.187, 0.1875, 0.1876] },
  { symbol: 'ICP', pair: 'ICP/USD', price: '9.34', source: 'Pyth Network', change: '+5.1%', isPositive: true, sparkline: [8.9, 9.0, 9.1, 9.2, 9.34] },
];

export const featuredAssets = allAssets.slice(0, 7); // First 7 for homepage

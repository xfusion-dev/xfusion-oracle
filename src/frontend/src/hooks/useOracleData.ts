import { useQuery } from '@tanstack/react-query';
import { oracleClient } from '../services/oracle';
import type { Price, OracleMetrics } from '../services/oracle';

// Hook to fetch all symbols
export const useOracleSymbols = () => {
  return useQuery({
    queryKey: ['oracle', 'symbols'],
    queryFn: () => oracleClient.getAllSymbols(),
    refetchInterval: 10000, // Refetch every 10 seconds
  });
};

// Hook to fetch prices for all symbols
export const useOraclePrices = () => {
  const { data: symbols } = useOracleSymbols();
  
  return useQuery({
    queryKey: ['oracle', 'prices', symbols],
    queryFn: async () => {
      if (!symbols || symbols.length === 0) return [];
      const prices = await oracleClient.getPrices(symbols);
      
      // Combine symbols with their prices
      const assetsWithPrices = symbols.map((symbol, index) => ({
        symbol,
        pair: `${symbol}/USD`,
        price: prices[index],
        source: prices[index]?.source || 'N/A',
      }));

      // Sort by price descending (assets with prices first, then by price value)
      return assetsWithPrices.sort((a, b) => {
        // Assets with no price go to the end
        if (!a.price && !b.price) return 0;
        if (!a.price) return 1;
        if (!b.price) return -1;
        
        // Sort by price value descending
        return Number(b.price.value) - Number(a.price.value);
      });
    },
    enabled: !!symbols && symbols.length > 0,
    refetchInterval: 2000, // Refetch every 2 seconds to match update frequency
  });
};

// Hook to fetch oracle metrics
export const useOracleMetrics = () => {
  return useQuery({
    queryKey: ['oracle', 'metrics'],
    queryFn: () => oracleClient.getMetrics(),
    refetchInterval: 5000, // Refetch every 5 seconds
  });
};

// Hook to fetch featured assets (first 7 for homepage)
export const useFeaturedAssets = () => {
  const { data: allAssets, ...rest } = useOraclePrices();
  
  return {
    ...rest,
    data: allAssets?.slice(0, 7) || [],
  };
};

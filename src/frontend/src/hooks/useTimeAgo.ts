import { useState, useEffect } from 'react';
import { formatTimeAgo } from '../services/oracle';

// Hook that updates time ago every second for live countdown
export const useTimeAgo = (timestamp: bigint | null) => {
  const [timeAgo, setTimeAgo] = useState<string>('');

  useEffect(() => {
    if (!timestamp) {
      setTimeAgo('No data');
      return;
    }

    const updateTimeAgo = () => {
      setTimeAgo(formatTimeAgo(timestamp));
    };

    // Update immediately
    updateTimeAgo();

    // Update every second
    const interval = setInterval(updateTimeAgo, 1000);

    return () => clearInterval(interval);
  }, [timestamp]);

  return timeAgo;
};

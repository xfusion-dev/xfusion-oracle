import { XLogo } from '../ui/XLogo';
import { LinkedInLogo } from '../ui/LinkedInLogo';

export default function Footer() {
  return (
    <footer className="hidden md:block border-t border-primary px-6 py-8 bg-surface">
      <div className="max-w-7xl mx-auto">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-6">
            <p className="text-caption font-mono">
              XFUSION | Oracles
            </p>
            <div className="flex items-center gap-4">
              <a href="https://x.com/xfusion_finance" target="_blank" rel="noopener noreferrer" className="text-tertiary hover:text-primary transition-colors">
                <XLogo className="w-5 h-5" />
              </a>
              <a href="https://www.linkedin.com/company/xfusion-finance/" target="_blank" rel="noopener noreferrer" className="text-tertiary hover:text-primary transition-colors">
                <LinkedInLogo className="w-5 h-5" />
              </a>
            </div>
          </div>

          <div className="flex items-center gap-3 text-caption font-mono">
            <span>Powered by</span>
            <a href="https://xfusion.finance" target="_blank" rel="noopener noreferrer">
              <span className="text-primary font-semibold">XFUSION</span>
            </a>
          </div>
        </div>
      </div>
    </footer>
  );
} 
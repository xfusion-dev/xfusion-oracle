import { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';
import { ExternalLink, Menu, X } from 'lucide-react';


interface HeaderProps {
  showHero?: boolean;
}

export default function Header({ showHero = false }: HeaderProps) {
  const [isScrolled, setIsScrolled] = useState(!showHero);
  const [showAuthModal, setShowAuthModal] = useState(false);
  const [isMobileMenuOpen, setIsMobileMenuOpen] = useState(false);

  useEffect(() => {
    // Only set up scroll listener on homepage with hero
    if (!showHero) {
      // For all non-homepage pages, always show scrolled state
      setIsScrolled(true);
      return;
    }

    // Homepage with hero - handle scroll
    const handleScroll = () => {
      setIsScrolled(window.scrollY > 50);
    };

    window.addEventListener('scroll', handleScroll);
    return () => {
      window.removeEventListener('scroll', handleScroll);
    };
  }, [showHero]);

  const closeMobileMenu = () => {
    setIsMobileMenuOpen(false);
  };

  return (
    <>
      <nav className={`nav-main ${isScrolled ? 'scrolled' : ''}`}>
        <div className="nav-content">
          {/* Left side - Logo and Mobile Menu Button */}
          <div className="flex items-center gap-4">
            {/* Mobile Menu Button */}
            <button
              onClick={() => { setIsMobileMenuOpen(!isMobileMenuOpen); }}
              className="md:hidden p-2 text-tertiary hover:text-primary transition-colors"
            >
              {isMobileMenuOpen ? (
                <X className="w-5 h-5" />
              ) : (
                <Menu className="w-5 h-5" />
              )}
            </button>

            {/* Logo */}
            <Link to="/" className="nav-logo">
              <div className="nav-logo-icon">
                <span className="text-lg font-bold">X</span>
              </div>
              <div className="flex flex-col -space-y-1">
                <span>Fusion</span>
                <span className="text-unique uppercase" style={{ fontSize: '11px', letterSpacing: '0.15em' }}>Oracles</span>
              </div>
            </Link>
          </div>
        
          {/* Desktop Menu */}
          <div className="nav-menu hidden md:flex">
            <a href="/assets">Assets</a>
            <a href="https://docs.xfusion.finance/oracle/getting-started" target="_blank" className="flex items-center gap-1">
              Docs
              <ExternalLink className="w-4 h-4" />
            </a>
          </div>
        
              
        </div>
          

        {/* Mobile Menu Overlay */}
        {isMobileMenuOpen && (
          <div className="md:hidden absolute top-full left-0 right-0 bg-surface border-t border-primary z-50">
            <div className="flex flex-col py-4">
              <a 
                href="/assets" 
                onClick={closeMobileMenu}
                className="px-6 py-3 text-tertiary hover:text-primary transition-colors border-b border-primary/20"
              >
                Assets
              </a>
              <a 
                href="/docs" 
                onClick={closeMobileMenu}
                className="px-6 py-3 text-tertiary hover:text-primary transition-colors border-b border-primary/20 flex items-center gap-2"
              >
                Docs
                <ExternalLink className="w-4 h-4" />
              </a>
            </div>
          </div>
        )}
      </nav>

    </>
  );
} 
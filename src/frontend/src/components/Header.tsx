function Header() {
  return (
    <nav className="nav-main">
      <div className="nav-content">
        {/* Logo */}
        <a href="/" className="nav-logo">
          <div className="nav-logo-icon">
            <span className="text-lg font-bold">X</span>
          </div>
          <span>Fusion</span>
        </a>
      
        {/* Desktop Menu */}
        <div className="nav-menu hidden md:flex">
          <a href="#features">Features</a>
          <a href="#bundles">Bundles</a>
          <a href="#assets">Assets</a>
          <span className="nav-menu-disabled">
            Documentation
            <svg className="w-4 h-4 ml-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14" />
            </svg>
          </span>
        </div>
      
        {/* Right side - Auth */}
        <div className="nav-actions">
          <button className="btn-outline-unique">
            Sign In
          </button>
        </div>
      </div>
    </nav>
  );
}

export default Header;

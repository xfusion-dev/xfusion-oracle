import { Routes, Route } from 'react-router-dom';
import Layout from './components/layout/Layout';
import ScrollToTop from './components/layout/ScrollToTop';
import HomePage from './pages/HomePage';
import AssetsPage from './pages/AssetsPage';

function App() {
  return (
    <Layout>
      <ScrollToTop />
      <Routes>
        <Route path="/" element={<HomePage />} />
        <Route path="/assets" element={<AssetsPage />} />
      </Routes>
    </Layout>
  );
}

export default App;

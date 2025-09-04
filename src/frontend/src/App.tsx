import Layout from './components/layout/Layout';
import ScrollToTop from './components/layout/ScrollToTop';
import HomePage from './pages/HomePage';


function App() {
  return (
    <Layout>
        <ScrollToTop />
        <HomePage />
    </Layout>
  );
}

export default App;

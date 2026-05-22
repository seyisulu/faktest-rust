import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import TakeTest from './pages/TakeTest';
import Dashboard from './pages/Dashboard';

function App() {
  return (
    <Router>
      <Routes>
        <Route path="/" element={<Dashboard />} />
        <Route path="/test/:testId" element={<TakeTest />} />
      </Routes>
    </Router>
  );
}

export default App;
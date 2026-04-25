import { BrowserRouter as Router, Routes, Route, Link } from 'react-router-dom';
import ExplorerPage from './ExplorerPage';
import AboutPage from './AboutPage';

function Navbar() {
    return (
        <nav className="sticky top-0 z-50 bg-[#F4F0E6] border-b-2 border-[#4A453F] px-8 py-4 flex justify-between items-center">
            <div className="font-black text-2xl text-[#4A453F] tracking-wide">
                DOM Explorer
            </div>
            <div className="flex gap-8 font-bold text-[#4A453F]">
                <Link to="/" className="hover:text-[#728C69] hover:-translate-y-0.5 transition-transform">
                    [ Penjelajahan ]
                </Link>
                <Link to="/about" className="hover:text-[#728C69] hover:-translate-y-0.5 transition-transform">
                    [ Tentang Kami ]
                </Link>
            </div>
        </nav>
    );
}

export default function App() {
    return (
        <Router>
            <div className="min-h-screen bg-[#F4F0E6] text-[#4A453F] font-sans selection:bg-[#728C69] selection:text-[#FCFAF5]">
                <Navbar />
                <Routes>
                    <Route path="/" element={<ExplorerPage />} />
                    <Route path="/about" element={<AboutPage />} />
                </Routes>
            </div>
        </Router>
    );
}

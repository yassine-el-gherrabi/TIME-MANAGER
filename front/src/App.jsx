import './App.css';

function App() {
  return (
    <div className="min-h-screen bg-background text-foreground">
      <div className="container mx-auto px-4 py-8">
        <header className="text-center mb-8">
          <h1 className="text-4xl font-bold text-primary mb-2">Time Manager</h1>
          <p className="text-muted-foreground">Professional Time Management Application</p>
        </header>

        <main className="max-w-2xl mx-auto">
          <div className="bg-card border border-border rounded-lg p-6 shadow-sm">
            <h2 className="text-2xl font-semibold mb-4">Welcome to Time Manager</h2>
            <p className="text-muted-foreground mb-4">
              This is the frontend setup for the Time Manager application.
            </p>
            <div className="space-y-2">
              <p className="text-sm">✅ React with Tailwind CSS configured</p>
              <p className="text-sm">✅ Axios configured with interceptors</p>
              <p className="text-sm">✅ React Router ready</p>
              <p className="text-sm">✅ ESLint and Prettier enabled</p>
            </div>
          </div>

          <div className="mt-6 text-center">
            <button className="bg-primary text-primary-foreground px-6 py-2 rounded-md hover:opacity-90 transition-opacity">
              Get Started
            </button>
          </div>
        </main>
      </div>
    </div>
  );
}

export default App;

import { Button } from '@/components/ui/button'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'

function App() {
  return (
    <div className="min-h-screen bg-background flex items-center justify-center p-4">
      <Card className="w-full max-w-md">
        <CardHeader>
          <CardTitle>Time Manager</CardTitle>
          <CardDescription>SaaS Workforce Management Platform</CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <p className="text-sm text-muted-foreground">
            Frontend React/TypeScript application successfully configured with:
          </p>
          <ul className="text-sm space-y-1 list-disc list-inside text-muted-foreground">
            <li>React 18 + TypeScript 5</li>
            <li>Vite 5 build tool</li>
            <li>Tailwind CSS styling</li>
            <li>Shadcn/UI components</li>
          </ul>
          <Button className="w-full">Get Started</Button>
        </CardContent>
      </Card>
    </div>
  )
}

export default App

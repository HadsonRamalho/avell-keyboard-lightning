import { useState } from "react";
import { ColorControls } from "./components/ColorControls";
import { KeyboardPreview } from "./components/KeyboardPreview";

export default function App() {
  const [selectedColor, setSelectedColor] = useState("#169E69");

  return (
    <main className="min-h-screen bg-background p-4 md:p-8 dark">
      <div className="mx-auto max-w-7xl">
        <header className="mb-8 md:mb-12 text-center">
          <div className="flex items-center justify-center gap-3 mb-4">
            <div className="h-10 w-10 rounded-lg bg-gradient-to-br from-emerald-500 to-blue-600 shadow-lg" />
            <h1 className="text-3xl md:text-4xl font-bold bg-gradient-to-r from-emerald-400 to-blue-600 bg-clip-text text-transparent">
              Keyboard Lightning Manager for Avell Storm 450r
            </h1>
          </div>
          <p className="text-muted-foreground text-base md:text-lg">
            Customize your Avell Storm 450r keyboard's RGB lighting.
          </p>
        </header>

        <div className="grid gap-6 lg:grid-cols-[1fr,400px]">
          <div className="order-2 lg:order-1">
            <KeyboardPreview color={selectedColor} />
          </div>

          <div className="order-1 lg:order-2">
            <ColorControls
              selectedColor={selectedColor}
              onColorChange={setSelectedColor}
            />
          </div>
        </div>
      </div>
    </main>
  );
}

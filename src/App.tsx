import { invoke } from "@tauri-apps/api/core";
import { Minimize2, X } from "lucide-react";
import { useState } from "react";
import { ColorControls } from "./components/ColorControls";
import { KeyboardPreview } from "./components/KeyboardPreview";
import { Button } from "./components/ui/button";

export default function App() {
	const [selectedColor, setSelectedColor] = useState("#169E69");

	const handleMinimizeToTray = async () => {
		try {
			await invoke("hide_main_window");
		} catch (error) {
			console.error("Error minimizing to tray:", error);
		}
	};

	const handleClose = async () => {
		try {
			await invoke("hide_main_window");
		} catch (error) {
			console.error("Error closing to tray:", error);
		}
	};

	return (
		<main className="min-h-screen bg-background p-4 md:p-8 dark">
			<div className="mx-auto max-w-7xl">
				<header className="mb-8 md:mb-12">
					<div className="flex items-center justify-between mb-4">
						<div className="flex items-center gap-3">
							<div className="h-10 w-10 rounded-lg bg-gradient-to-br from-emerald-500 to-blue-600 shadow-lg" />
							<h1 className="text-2xl md:text-3xl font-bold bg-gradient-to-r from-emerald-400 to-blue-600 bg-clip-text text-transparent">
								Keyboard Lightning Manager for Avell Storm 450r
							</h1>
						</div>
						<div className="flex items-center gap-2">
							<Button
								variant="ghost"
								size="icon"
								onClick={handleMinimizeToTray}
								title="Minimize to System Tray"
								className="h-8 w-8 hover:bg-muted"
							>
								<Minimize2 className="h-4 w-4" />
							</Button>
							<Button
								variant="ghost"
								size="icon"
								onClick={handleClose}
								title="Close to System Tray"
								className="h-8 w-8  hover:bg-red-500/10 hover:text-red-500"
							>
								<X className="h-4 w-4" />
							</Button>
						</div>
					</div>
					<div className="text-center">
						<p className="text-muted-foreground text-base md:text-lg">
							Customize your Avell Storm 450r keyboard's RGB lighting.
						</p>
						<p className="text-xs text-muted-foreground mt-2">
							App will continue running in system tray when minimized or closed
						</p>
					</div>
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

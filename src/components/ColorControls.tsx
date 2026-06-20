import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Switch } from "@/components/ui/switch";

interface ColorControlsProps {
	selectedColor: string;
	onColorChange: (color: string) => void;
}

interface ModuleStatus {
	id: string;
	name: string;
	available: boolean;
	enabled: boolean;
}

const presetColors = [
	{ name: "Purple", value: "#8B5CF6" },
	{ name: "Blue", value: "#3B82F6" },
	{ name: "Cyan", value: "#06B6D4" },
	{ name: "Green", value: "#10B981" },
	{ name: "Yellow", value: "#F59E0B" },
	{ name: "Orange", value: "#F97316" },
	{ name: "Red", value: "#EF4444" },
	{ name: "Pink", value: "#EC4899" },
	{ name: "Magenta", value: "#D946EF" },
	{ name: "White", value: "#F3F4F6" },
];

function hexToRgb(hex: string): { r: number; g: number; b: number } {
	const result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
	return result
		? {
				r: Number.parseInt(result[1], 16),
				g: Number.parseInt(result[2], 16),
				b: Number.parseInt(result[3], 16),
			}
		: { r: 0, g: 0, b: 0 };
}

function rgbToHex(r: number, g: number, b: number): string {
	return (
		"#" +
		[r, g, b]
			.map((x) => {
				const hex = x.toString(16);
				return hex.length === 1 ? `0${hex}` : hex;
			})
			.join("")
	);
}

export function ColorControls({
	selectedColor,
	onColorChange,
}: ColorControlsProps) {
	const rgb = hexToRgb(selectedColor);
	const [hexInput, setHexInput] = useState(selectedColor);
	const [isScreenCaptureActive, setIsScreenCaptureActive] = useState(false);
	const [isLoadingScreenCapture, setIsLoadingScreenCapture] = useState(false);
	const [isRainbowEffectActive, setIsRainbowEffectActive] = useState(false);
	const [isLoadingRainbowEffect, setIsLoadingRainbowEffect] = useState(false);
	const [isBreathEffectActive, setIsBreathEffectActive] = useState(false);
	const [isLoadingBreathEffect, setIsLoadingBreathEffect] = useState(false);
	const [isTypingEffectActive, setIsTypingEffectActive] = useState(false);
	const [isLoadingTypingEffect, setIsLoadingTypingEffect] = useState(false);
	const [modules, setModules] = useState<ModuleStatus[]>([]);

	useEffect(() => {
		const checkStatus = async () => {
			try {
				const screenActive = (await invoke(
					"is_screen_capture_active",
				)) as boolean;
				const rainbowActive = (await invoke(
					"is_rainbow_effect_active",
				)) as boolean;
				const modulesData = (await invoke("get_modules_status")) as ModuleStatus[];
				setIsScreenCaptureActive(screenActive);
				setIsRainbowEffectActive(rainbowActive);
				setModules(modulesData);
			} catch (error) {
				console.error("Error checking status:", error);
			}
		};

		checkStatus();
	}, []);

	const handleRainbowEffectToggle = async () => {
		if (isLoadingRainbowEffect) return;

		setIsLoadingRainbowEffect(true);

		try {
			if (isRainbowEffectActive) {
				await invoke("stop_rainbow_effect");
				setIsRainbowEffectActive(false);
			} else {
				await invoke("start_rainbow_effect");
				setIsRainbowEffectActive(true);
				if (isScreenCaptureActive) {
					setIsScreenCaptureActive(false);
				}
				if (isBreathEffectActive) {
					setIsBreathEffectActive(false);
				}
				if (isTypingEffectActive) {
					setIsTypingEffectActive(false);
				}
			}
		} catch (error) {
			console.error("Error toggling rainbow effect:", error);
			alert(error);
		} finally {
			setIsLoadingRainbowEffect(false);
		}
	};

	const handleBreathEffectToggle = async () => {
		if (isLoadingBreathEffect) return;

		setIsLoadingBreathEffect(true);

		try {
			if (isBreathEffectActive) {
				await invoke("stop_breath_effect");
				setIsBreathEffectActive(false);
			} else {
				await invoke("start_breath_effect", {
					red: rgb.r,
					green: rgb.g,
					blue: rgb.b,
				});
				setIsBreathEffectActive(true);
				if (isScreenCaptureActive) {
					setIsScreenCaptureActive(false);
				}
				if (isRainbowEffectActive) {
					setIsRainbowEffectActive(false);
				}
			}
		} catch (error) {
			console.error("Error toggling breath effect:", error);
			alert(error);
		} finally {
			setIsLoadingBreathEffect(false);
		}
	};

	const handleTypingEffectToggle = async () => {
		if (isLoadingTypingEffect) return;

		setIsLoadingTypingEffect(true);

		try {
			if (isTypingEffectActive) {
				await invoke("stop_typing_heatmap");
				setIsTypingEffectActive(false);
			} else {
				await invoke("start_typing_heatmap", {
					red: rgb.r,
					green: rgb.g,
					blue: rgb.b,
				});
				setIsTypingEffectActive(true);
				if (isScreenCaptureActive) {
					setIsScreenCaptureActive(false);
				}
				if (isRainbowEffectActive) {
					setIsRainbowEffectActive(false);
				}
				if (isBreathEffectActive) {
					setIsBreathEffectActive(false);
				}
			}
		} catch (error) {
			console.error("Error toggling typing effect:", error);
			alert(error);
		} finally {
			setIsLoadingTypingEffect(false);
		}
	};

	const handleRgbChange = async (channel: "r" | "g" | "b", value: string) => {
		const numValue = Math.max(0, Math.min(255, Number.parseInt(value) || 0));
		const newRgb = { ...rgb, [channel]: numValue };
		const newHex = rgbToHex(newRgb.r, newRgb.g, newRgb.b);
		onColorChange(newHex);
		setHexInput(newHex);
	};

	const handleHexChange = (value: string) => {
		setHexInput(value);
		if (/^#[0-9A-F]{6}$/i.test(value)) {
			onColorChange(value);
		}
	};

	const handleConfirm = async () => {
		try {
			await invoke("update_keyboard_color", {
				red: rgb.r,
				green: rgb.g,
				blue: rgb.b,
			});
		} catch (error) {
			console.error(error);
			alert(error);
		}
	};

	const handleScreenCaptureToggle = async () => {
		if (isLoadingScreenCapture) return;

		setIsLoadingScreenCapture(true);

		try {
			if (isScreenCaptureActive) {
				await invoke("stop_screen_capture");
				setIsScreenCaptureActive(false);
			} else {
				await invoke("start_screen_capture");
				setIsScreenCaptureActive(true);
				if (isRainbowEffectActive) {
					setIsRainbowEffectActive(false);
				}
				if (isBreathEffectActive) {
					setIsBreathEffectActive(false);
				}
				if (isTypingEffectActive) {
					setIsTypingEffectActive(false);
				}
			}
		} catch (error) {
			console.error("Error toggling screen capture:", error);
			alert(error);
		} finally {
			setIsLoadingScreenCapture(false);
		}
	};

	const handleModuleToggle = async (id: string, enabled: boolean) => {
		try {
			await invoke("set_module_enabled", { id, enabled });
			setModules((prev) =>
				prev.map((m) => (m.id === id ? { ...m, enabled } : m))
			);
		} catch (error) {
			console.error("Error toggling module:", error);
			alert(error);
		}
	};

	return (
		<Card className="p-6 space-y-6 bg-card">
			<div>
				<h2 className="text-lg font-semibold text-card-foreground mb-4">
					Color Controls
				</h2>
			</div>

			<div className="space-y-3">
				<div className="flex items-center justify-between">
					<div className="space-y-1">
						<Label className="text-sm font-medium text-card-foreground">
							Rainbow Effect
						</Label>
						<p className="text-xs text-muted-foreground">
							Smooth rainbow color cycling effect
						</p>
					</div>
					<Switch
						checked={isRainbowEffectActive}
						onCheckedChange={handleRainbowEffectToggle}
						disabled={isLoadingRainbowEffect}
					/>
				</div>

				<div className="flex items-center justify-between">
					<div className="space-y-1">
						<Label className="text-sm font-medium text-card-foreground">
							Screen Color Sync
						</Label>
						<p className="text-xs text-muted-foreground">
							Automatically sync keyboard color with screen content
						</p>
					</div>
					<Switch
						checked={isScreenCaptureActive}
						onCheckedChange={handleScreenCaptureToggle}
						disabled={isLoadingScreenCapture}
					/>
				</div>

				<div className="flex items-center justify-between">
					<div className="space-y-1">
						<Label className="text-sm font-medium text-card-foreground">
							Breath Effect
						</Label>
						<p className="text-xs text-muted-foreground">
							A simple breath effect
						</p>
					</div>
					<Switch
						checked={isBreathEffectActive}
						onCheckedChange={handleBreathEffectToggle}
						disabled={isLoadingBreathEffect}
					/>
				</div>

				<div className="flex items-center justify-between">
					<div className="space-y-1">
						<Label className="text-sm font-medium text-card-foreground">
							Typing Heatmap Effect
						</Label>
						<p className="text-xs text-muted-foreground">
							Increase the keyboard color heat while typing
						</p>
					</div>
					<Switch
						checked={isTypingEffectActive}
						onCheckedChange={handleTypingEffectToggle}
						disabled={isLoadingTypingEffect}
					/>
				</div>
			</div>

			<div className="space-y-3">
				<Label className="text-sm font-medium text-card-foreground">
					Module Sync Integrations
				</Label>
				<p className="text-xs text-muted-foreground mb-2">
					Select which apps should sync with the keyboard color
				</p>
				<div className="space-y-2">
					{modules.map((module) => (
						<div
							key={module.id}
							className="flex items-center justify-between bg-secondary/30 p-2 rounded-md"
						>
							<div className="flex items-center space-x-2">
								<span
									className={`text-sm ${
										module.available
											? "text-card-foreground"
											: "text-muted-foreground line-through"
									}`}
								>
									{module.name}
								</span>
								{!module.available && (
									<span className="text-[10px] bg-muted text-muted-foreground px-1.5 py-0.5 rounded">
										Not Found
									</span>
								)}
							</div>
							<Switch
								checked={module.enabled && module.available}
								onCheckedChange={(checked) =>
									handleModuleToggle(module.id, checked)
								}
								disabled={!module.available}
							/>
						</div>
					))}
				</div>
			</div>

			<div
				className={`space-y-3 ${isScreenCaptureActive || isRainbowEffectActive ? "opacity-50 pointer-events-none" : ""}`}
			>
				<Label className="text-sm font-medium text-card-foreground">
					Predefined Colors
				</Label>
				<div className="flex flex-row w-full gap-2">
					{presetColors.map((preset) => (
						<Button
							key={preset.value}
							onClick={() => {
								onColorChange(preset.value);
								setHexInput(preset.value);
							}}
							className={`
                aspect-square rounded-lg transition-all duration-200
                hover:scale-110 hover:shadow-lg w-30
                ${
									selectedColor.toLowerCase() === preset.value.toLowerCase()
										? "ring-2 ring-primary ring-offset-2 ring-offset-card scale-110"
										: "hover:ring-2 hover:ring-primary/50"
								}
              `}
							style={{ backgroundColor: preset.value }}
							title={preset.name}
							disabled={isScreenCaptureActive}
						/>
					))}
				</div>
			</div>

			<div
				className={`space-y-3 ${isScreenCaptureActive || isRainbowEffectActive ? "opacity-50 pointer-events-none" : ""}`}
			>
				<Label
					htmlFor="color-picker"
					className="text-sm font-medium text-card-foreground"
				>
					Color Picker
				</Label>
				<div className="flex gap-3 items-center">
					<input
						id="color-picker"
						type="color"
						value={selectedColor}
						onChange={(e) => {
							onColorChange(e.target.value);
							setHexInput(e.target.value);
						}}
						className="h-12 w-full rounded-lg cursor-pointer bg-secondary border-2 border-border"
						disabled={isScreenCaptureActive}
					/>
				</div>
			</div>

			<div
				className={`space-y-3 ${isScreenCaptureActive || isRainbowEffectActive ? "opacity-50 pointer-events-none" : ""}`}
			>
				<Label className="text-sm font-medium text-card-foreground">RGB</Label>
				<div className="grid grid-cols-3 gap-3">
					<div className="space-y-2">
						<Label htmlFor="r-input" className="text-xs text-muted-foreground">
							R
						</Label>
						<Input
							id="r-input"
							type="number"
							min="0"
							max="255"
							value={rgb.r}
							onChange={(e) => handleRgbChange("r", e.target.value)}
							className="bg-secondary border-border text-card-foreground"
							disabled={isScreenCaptureActive}
						/>
					</div>
					<div className="space-y-2">
						<Label htmlFor="g-input" className="text-xs text-muted-foreground">
							G
						</Label>
						<Input
							id="g-input"
							type="number"
							min="0"
							max="255"
							value={rgb.g}
							onChange={(e) => handleRgbChange("g", e.target.value)}
							className="bg-secondary border-border text-card-foreground"
							disabled={isScreenCaptureActive}
						/>
					</div>
					<div className="space-y-2">
						<Label htmlFor="b-input" className="text-xs text-muted-foreground">
							B
						</Label>
						<Input
							id="b-input"
							type="number"
							min="0"
							max="255"
							value={rgb.b}
							onChange={(e) => handleRgbChange("b", e.target.value)}
							className="bg-secondary border-border text-card-foreground"
							disabled={isScreenCaptureActive}
						/>
					</div>
				</div>
			</div>

			<div
				className={`space-y-3 ${isScreenCaptureActive || isRainbowEffectActive ? "opacity-50 pointer-events-none" : ""}`}
			>
				<Label
					htmlFor="hex-input"
					className="text-sm font-medium text-card-foreground"
				>
					HEX
				</Label>
				<Input
					id="hex-input"
					type="text"
					value={hexInput}
					onChange={(e) => handleHexChange(e.target.value.toUpperCase())}
					placeholder="#8B5CF6"
					className="bg-secondary border-border text-card-foreground font-mono"
					maxLength={7}
					disabled={isScreenCaptureActive}
				/>
			</div>

			<Button
				className="w-full bg-primary hover:bg-primary/90 text-primary-foreground"
				size="lg"
				onClick={() => {
					handleConfirm();
				}}
				disabled={isScreenCaptureActive || isRainbowEffectActive}
			>
				{isScreenCaptureActive
					? "Screen Sync Active"
					: isRainbowEffectActive
						? "Rainbow Effect Active"
						: "Apply Configuration"}
			</Button>
		</Card>
	);
}

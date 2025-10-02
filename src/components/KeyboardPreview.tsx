interface KeyboardPreviewProps {
  color: string;
}

const keyboardLayout = [
  [
    "Esc",
    "F1",
    "F2",
    "F3",
    "F4",
    "F5",
    "F6",
    "F7",
    "F8",
    "F9",
    "F10",
    "F11",
    "F12",
    "Prt Sc",
    "Del",
    "End",
    "+",
    "-",
  ],
  [
    "`",
    "1",
    "2",
    "3",
    "4",
    "5",
    "6",
    "7",
    "8",
    "9",
    "0",
    "-",
    "=",
    "Backspace",
    "Num",
    "/",
    "*",
  ],
  [
    "Tab",
    "Q",
    "W",
    "E",
    "R",
    "T",
    "Y",
    "U",
    "I",
    "O",
    "P",
    "´`",
    "[{",
    "Enter",
    "7",
    "8",
    "9",
  ],
  [
    "Fixa",
    "A",
    "S",
    "D",
    "F",
    "G",
    "H",
    "J",
    "K",
    "L",
    "Ç",
    "~",
    "]}",
    "Enter",
    "4",
    "5",
    "6",
  ],
  [
    "Shift",
    "\\|",
    "Z",
    "X",
    "C",
    "V",
    "B",
    "N",
    "M",
    ",<",
    ".>",
    ";:",
    "Shift",
    "↑",
    "1",
    "2",
    "3",
  ],
  [
    "Ctrl",
    "Fn",
    "Win",
    "Alt",
    "Space",
    "Alt Gr",
    "Ops",
    "Ctrl",
    "←",
    "↓",
    "→",
    "0",
    ", Del",
  ],
];

export function KeyboardPreview({ color }: KeyboardPreviewProps) {
  return (
    <div className="rounded-xl bg-card p-6 md:p-8 shadow-2xl">
      <h2 className="text-lg font-semibold text-card-foreground mb-6">
        Preview
      </h2>

      <div className="hidden md:block bg-secondary/50 rounded-lg p-4 md:p-6">
        <div className="space-y-2">
          {keyboardLayout.map((row, rowIndex) => (
            <div key={rowIndex} className="flex gap-1 md:gap-2 justify-center">
              {row.map((key, keyIndex) => {
                const isEnter = key === "Enter";
                const isShift = key === "Shift";
                const isWide =
                  key === "Backspace" ||
                  key === "Fixa" ||
                  key === "Tab" ||
                  key === "Caps";
                const isExtraWide = key === "Space";

                return (
                  <div
                    key={`${rowIndex}-${keyIndex}`}
                    className={`
                      relative rounded bg-secondary/80 backdrop-blur-sm
                      flex items-center justify-center
                      text-[8px] md:text-xs font-medium text-secondary-foreground
                      transition-all duration-300
                      ${isExtraWide ? "w-32 md:w-70" : isWide ? "w-16 md:w-22" : isEnter ? "w-10 md:w-10" : isShift ? "w-16 md:w-16" : "w-8 md:w-10"}
                      h-8 md:h-10
                    `}
                    style={{
                      boxShadow: `inset 0 0 0 2px ${color}, 0 0 20px ${color}40`,
                    }}
                  >
                    {key}
                  </div>
                );
              })}
            </div>
          ))}
        </div>
      </div>

      <div className="mt-4 flex items-center gap-2 text-xs text-muted-foreground">
        <div
          className="w-3 h-3 rounded-full"
          style={{ backgroundColor: color }}
        />
        <span>Current color: {color.toUpperCase()}</span>
      </div>
    </div>
  );
}

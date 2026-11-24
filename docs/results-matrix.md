# قالب جدول النتائج: Terminals × AI CLI tools

كل خلية = نتيجة تشغيل `yamin-test eval` لتلك التركيبة: **ناجح / مُقيَّم (النسبة)**. الحالات المتخطاة (skip) لا تدخل في النسبة.

- العمود **none** يعني shell عاديًا بلا أداة AI: يقيس الـ terminal وحده.
- عمود أداة AI يعني أن الحالات عُرضت **من داخل** واجهة تلك الأداة. مثلًا: طلبت من الأداة تشغيل `yamin-test show` أو عرض النص نفسه في ردها. الفرق بين هذا العمود وعمود none هو أثر طبقة العرض في الأداة (TUI، تغليف الأسطر، markdown).

## طريقة الملء

```sh
# لكل تركيبة terminal × أداة:
yamin-test eval --terminal "WezTerm" --cli "none" --out reports
yamin-test eval --terminal "WezTerm" --cli "Claude Code" --out reports

# بعد جمع التقارير:
yamin-test matrix --reports reports --out docs/results.md
```

`matrix` يولّد الجدول تلقائيًا من ملفات JSON. إذا وُجد أكثر من تقرير للتركيبة نفسها، يُعتمد الأحدث تاريخًا. الجدول أدناه قالب يدوي للتخطيط وتتبّع التقدم.

## الجدول

| Terminal (المحرك / النظام) | none | Claude Code | Codex CLI | Gemini CLI | Aider | GitHub Copilot CLI |
|---|---|---|---|---|---|---|
| Windows Terminal (Windows) | | | | | | |
| conhost (Windows) | | | | | | |
| WezTerm | | | | | | |
| Alacritty | | | | | | |
| Kitty | | | | | | |
| Ghostty | | | | | | |
| GNOME Terminal (VTE) | | | | | | |
| Konsole | | | | | | |
| mlterm | | | | | | |
| foot | | | | | | |
| iTerm2 (macOS) | | | | | | |
| Terminal.app (macOS) | | | | | | |
| VS Code integrated terminal (xterm.js) | | | | | | |

## ما يُسجَّل مع كل خلية

| الحقل | مثال |
|---|---|
| إصدار الـ terminal | WezTerm 20240203 |
| إصدار أداة AI | 2.x |
| نظام التشغيل | Windows 11 26300 |
| الخط | Cascadia Mono / DejaVu Sans Mono |
| إعدادات BiDi | مثل `bidi_enabled = true` في WezTerm، أو تفعيل BiDi في Konsole |
| تقرير التشغيل | `reports/yamin-wezterm-none-2026-11-24.md` |

> الأسماء في الجدول اقتراحات. أضف أو احذف صفوفًا وأعمدة حسب ما تختبره. أداة `matrix` لا تعتمد على هذه القائمة، وتبني الأعمدة من التقارير الموجودة.

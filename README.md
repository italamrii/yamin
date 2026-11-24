# yamin

محرك BiDi + Arabic shaping لشبكة خلايا الـ terminal، مكتوب بـ Rust.

- **`yamin-test`:** أداة مطابقة من 40 حالة.
- **`yamin`:** المكتبة، وتطبّق القرارات المعتمدة في [docs/spec.md](docs/spec.md). نجحت في الحالات الأربعين كلها.
- **`yamin-cat`:** أداة تمرّر النص عبر المكتبة، فيظهر العربي صحيحًا في أي terminal، حتى لو ما يدعم BiDi.

## yamin-cat

```sh
cargo run -q -p yamin-cat -- examples/arabic.txt         # ملف
some-command | cargo run -q -p yamin-cat                 # أو مخرجات أي أمر
cargo run -q -p yamin-cat -- --dir rtl --width 40 notes.txt
```

| الخيار | المعنى |
|---|---|
| `--dir ltr\|rtl\|auto` | اتجاه الفقرة. الافتراضي `auto`: أول حرف strong يحدد الاتجاه |
| `--width N` | العرض اللي تلتف عنده السطور وتُحاذى له سطور RTL. الافتراضي عرض الـ terminal، أو 80 إذا ما عُرف |
| `--no-align` | لا تُحاذى سطور RTL لليمين |
| `--protect` | يحيط كل سطر بـ LRO…PDF، للـ terminals اللي تطبّق BiDi بنفسها |

كل سطر يُعامل كفقرة مستقلة. الألوان وتسلسلات الهروب تُحذف في هذا الإصدار.

**ملاحظة لـ Windows PowerShell 5.1:** عند تمرير نص عبر `|` إلى برنامج خارجي، يرسله PowerShell بترميز ASCII، فيصير العربي `?`. الحل: مرّر اسم الملف مباشرة، أو نفّذ هذا السطر قبل استخدام `|`:

```powershell
$OutputEncoding = [Text.UTF8Encoding]::new()
```

## yamin (المكتبة)

```rust
use yamin::{cells_from_str, layout, Direction, Props};

let cells = cells_from_str("مرحبا world!");          // الـ model: خلايا بالترتيب المنطقي
let props = Props { direction: Direction::Rtl, ..Props::default() };
let l = layout(&cells, 80, props);                     // فقرة واحدة ملتفة على 80 عمودًا
for vc in &l.rows[0] {
    // vc.logical: رقم الخلية في الـ model
    // vc.glyph: ما يُرسم (بعد الـ shaping والـ mirroring)
    // vc.rtl: يحدد جهة المؤشر
}
```

| الوضع | ما تفعله المكتبة |
|---|---|
| `Mode::Implicit` (الافتراضي) | UAX#9 على الفقرة كاملة (عبر `unicode-bidi`) ثم إعادة ترتيب كل صف (L1 وL2)، وshaping منطقي على الفقرة (D3)، ولام-ألف على خليتين (D1)، وmirroring بقاعدة L4، والخلايا الممسوحة تذهب لنهاية الصف |
| `Mode::Explicit` | لا BiDi ولا shaping. في LTR يُرسم الصف كما هو، وفي RTL يُعكس الصف مع mirroring |
| `autodetect: true` | يُحدَّد الاتجاه من أول حرف strong في الفقرة (P2/P3)، وإن لم يوجد يُستخدم `direction` احتياطيًا. يعمل في implicit فقط، ومعطّل افتراضيًا (D5) |

## yamin-test

```sh
cargo run -p yamin-test -- list                       # الحالات الأربعون
cargo run -p yamin-test -- show --category lamalef    # السطر الفعلي + السطر المتوقع
cargo run -p yamin-test -- show --id mixed-05 --codepoints
cargo run -p yamin-test -- html --out reference.html  # صفحة مرجعية تُفتح في المتصفح
```

لكل حالة يطبع `show` سطرين:

1. **actual:** النص بترتيب logical، والـ terminal الذي تختبره هو من يرتّبه ويشكّله (implicit mode).
2. **expected:** النص مرتّبًا مسبقًا بترتيب visual وبحروف presentation forms. يُطبع في explicit mode (`CSI 8 l`) ومحاطًا بـ LRO…PDF.

قارن السطرين بصريًا. خيارات العرض:

- `--no-escapes`: بدون تسلسلات BDSM/SCP.
- `--no-controls`: بدون LRO/PDF.

### وضع التقييم

```sh
cargo run -p yamin-test -- eval --terminal "WezTerm" --cli none
cargo run -p yamin-test -- eval --terminal "WezTerm" --cli "Claude Code" --category paths
```

يعرض الحالات واحدة واحدة، وتجيب على كل حالة بـ `p` (pass) أو `f` (fail) أو `s` (skip) أو `q` (quit). عند الفشل يطلب ملاحظة اختيارية.

في النهاية يكتب تقريرين في `reports/`:

- `yamin-<terminal>-<cli>-<date>.json`
- `yamin-<terminal>-<cli>-<date>.md`

## ملاحظة لـ Windows

إذا كان مسار المشروع طويلًا، يفشل الـ linker بالخطأ `LNK1104` لأن المسار يتجاوز 260 حرفًا. الحل أن تجعل مجلد البناء في مسار قصير:

```powershell
$env:CARGO_TARGET_DIR = "$env:TEMP\yamin-target"
```

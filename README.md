# yamin

محرك BiDi + Arabic shaping لشبكة خلايا الـ terminal، مكتوب بـ Rust.

- **المرحلة 1 (الحالية):** `yamin-test`، أداة مطابقة من 40 حالة.
- **مكتبة `yamin` نفسها:** لم تبدأ بعد، وتنتظر الموافقة على [docs/spec.md](docs/spec.md).

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

## ملاحظة لـ Windows

إذا كان مسار المشروع طويلًا، يفشل الـ linker بالخطأ `LNK1104` لأن المسار يتجاوز 260 حرفًا. الحل أن تجعل مجلد البناء في مسار قصير:

```powershell
$env:CARGO_TARGET_DIR = "$env:TEMP\yamin-target"
```

#include "ape/ape.h"
// Intentionally minimal; shared lib symbol anchor if needed later. Keeping this
// translation unit ensures a stable symbol is exported even if the C++ API is
// header-only optimized in the future.
extern "C" void ape_export_anchor() {}

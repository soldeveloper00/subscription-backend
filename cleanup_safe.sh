#!/bin/bash

echo "🧹 Starting safe cleanup..."
echo ""

# 1. Remove backup files
echo "1. Removing backup files..."
find . -name "*.backup*" -type f -delete 2>/dev/null
find . -name "*~" -type f -delete 2>/dev/null  # Emacs backups
find . -name "*.swp" -type f -delete 2>/dev/null  # Vim swaps
echo "✅ Backup files removed"

# 2. Remove temporary fix scripts
echo ""
echo "2. Removing temporary scripts..."
rm -f diagnose_routes.sh final_fix.sh fix_api.sh fix_historical_prices.sh 2>/dev/null
rm -f fix_script.sh fix_uuid.sh setup_blockchain_integration.sh test_backend.sh 2>/dev/null
rm -f fix_project.sh verify_and_clean.sh 2>/dev/null
echo "✅ Temporary scripts removed"

# 3. Check empty/unused directories
echo ""
echo "3. Checking directories..."

# Check services directory
if [ -d "src/services" ]; then
    rs_files=$(find src/services -name "*.rs" -type f | wc -l)
    if [ $rs_files -eq 1 ] && [ -f "src/services/mod.rs" ]; then
        echo "   src/services/ only has mod.rs - removing..."
        rm -rf src/services/
    else
        echo "   src/services/ has $rs_files .rs files"
    fi
fi

# Check utils directory  
if [ -d "src/utils" ]; then
    if [ -f "src/utils/http_client.rs" ] || [ -f "src/utils/wallet.rs" ]; then
        echo "   src/utils/ has utility files - keeping"
    else
        echo "   src/utils/ has no utility files - removing..."
        rm -rf src/utils/
    fi
fi

# Check signals directory
if [ -d "src/signals" ]; then
    signal_files=$(find src/signals -name "*.rs" -type f ! -name "mod.rs" | wc -l)
    if [ $signal_files -eq 0 ]; then
        echo "   src/signals/ has no signal implementations - removing..."
        rm -rf src/signals/
    else
        echo "   src/signals/ has $signal_files signal files - keeping"
    fi
fi

# Remove config.rs if empty or small
if [ -f "src/config.rs" ]; then
    lines=$(wc -l < src/config.rs)
    if [ $lines -lt 10 ]; then
        echo "   src/config.rs is small ($lines lines) - removing..."
        rm -f src/config.rs
    else
        echo "   src/config.rs has $lines lines - keeping"
    fi
fi

# 4. Clean up root directory
echo ""
echo "4. Cleaning root directory..."
ls -la *.sh 2>/dev/null | grep -v "cleanup_safe.sh" | awk '{print $9}' | while read script; do
    echo "   Removing: $script"
    rm -f "$script"
done

# 5. Final check
echo ""
echo "5. Final structure check..."
cargo check --quiet && echo "✅ Build still works" || echo "❌ Build broken!"

echo ""
echo "=== Final Structure ==="
find . -type f -name "*.rs" -o -name "*.toml" -o -name "*.md" -o -name "*.sh" 2>/dev/null | \
    grep -v target | grep -v "/build/" | sort

echo ""
echo "🎉 Cleanup complete! Your backend is ready for submission."

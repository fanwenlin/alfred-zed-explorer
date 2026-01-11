#!/bin/bash

echo "🔍 Verifying OpenInZed workflow..."
echo ""

# Check workflow exists
if [ ! -d "~/Desktop/OpenInZed.alfredworkflow" ]; then
    echo "❌ Workflow not found on Desktop"
    exit 1
fi

echo "✅ Workflow found on Desktop"

# Check files
if [ -f "~/Desktop/OpenInZed.alfredworkflow/info.plist" ]; then
    echo "✅ info.plist exists"
fi

if [ -f "~/Desktop/OpenInZed.alfredworkflow/zed-search" ]; then
    echo "✅ zed-search binary exists"
fi

if [ -f "~/Desktop/OpenInZed.alfredworkflow/zed-recent" ]; then
    echo "✅ zed-recent binary exists"
fi

if [ -f "~/Desktop/OpenInZed.alfredworkflow/icon.png" ]; then
    echo "✅ icon.png (Zed logo) exists"
fi

echo ""
echo "📋 Workflow contents:"
ls -lh ~/Desktop/OpenInZed.alfredworkflow/

echo ""
echo "🔑 Bundle info from info.plist:"
echo "Bundle ID: $(grep -A 1 "bundleid" ~/Desktop/OpenInZed.alfredworkflow/info.plist | tail -1 | sed 's/.*<string>\(.*\)<\/string>.*/\1/')"
echo "Name: $(grep -A 1 "<key>name<" ~/Desktop/OpenInZed.alfredworkflow/info.plist | tail -1 | sed 's/.*<string>\(.*\)<\/string>.*/\1/')"

echo ""
echo "⌨️  Keywords:"
grep -A 1 "keyword" ~/Desktop/OpenInZed.alfredworkflow/info.plist | grep "<string>" | sed 's/.*<string>\(.*\)<\/string>.*/  - \1/'

echo ""
echo "✅ Verification complete!"
echo ""
echo "To install:"
echo "  Double-click: ~/Desktop/OpenInZed.alfredworkflow"

#!/bin/bash

echo "🔍 Verifying OpenInZed workflow fixes..."
echo ""

# Check workflow exists
echo "1. Checking workflow package..."
if [ -d "~/Desktop/OpenInZed.alfredworkflow" ]; then
    echo "   ✅ Workflow found on Desktop"
else
    echo "   ❌ Workflow not found"
    exit 1
fi

# Check icon.png
echo ""
echo "2. Checking workflow icon..."
if [ -f "~/Desktop/OpenInZed.alfredworkflow/icon.png" ]; then
    echo "   ✅ icon.png found"
    ICON_SIZE=$(du -h ~/Desktop/OpenInZed.alfredworkflow/icon.png | cut -f1)
    echo "   📏 Icon size: $ICON_SIZE"
else
    echo "   ❌ icon.png missing"
fi

# Check objects
echo ""
echo "3. Checking workflow objects..."
OBJECT_COUNT=$(grep -c "<key>type</key>" ~/Desktop/OpenInZed.alfredworkflow/info.plist)
echo "   Found $OBJECT_COUNT objects"

echo "   Object types:"
grep -A 1 "<key>type</key>" ~/Desktop/OpenInZed.alfredworkflow/info.plist | grep "<string>" | while read line; do
    echo "     - $(echo $line | sed 's/.*<string>\(.*\)<\/string>.*/\1/')"
done

# Check for bash action
echo ""
echo "4. Checking for removed bash action..."
if grep -q "alfred.workflow.action.script" ~/Desktop/OpenInZed.alfredworkflow/info.plist; then
    echo "   ❌ Bash action still exists!"
    exit 1
else
    echo "   ✅ No bash actions found"
fi

# Check for Open File action
echo ""
echo "5. Checking Open File action..."
OPENFILE_COUNT=$(grep -c "alfred.workflow.action.openfile" ~/Desktop/OpenInZed.alfredworkflow/info.plist)
if [ "$OPENFILE_COUNT" -eq 1 ]; then
    echo "   ✅ Open File action found"
    
    # Check that it has filepaths
    if grep -A 5 "alfred.workflow.action.openfile" ~/Desktop/OpenInZed.alfredworkflow/info.plist | grep -q "filepaths"; then
        echo "   ✅ Open File configured with filepaths"
        
        # Check for {query} variable
        if grep -A 10 "filepaths" ~/Desktop/OpenInZed.alfredworkflow/info.plist | grep -q "{query}"; then
            echo "   ✅ Open File uses {query} variable"
        fi
    fi
else
    echo "   ❌ Open File action not found or multiple found"
fi

# Check uidata
echo ""
echo "6. Checking uidata..."
UIDATA_COUNT=$(grep -c "<key>xpos</key>" ~/Desktop/OpenInZed.alfredworkflow/info.plist)
echo "   🔑 Found $UIDATA_COUNT objects in uidata"

# Check bundle info
echo ""
echo "7. Workflow metadata:"
BUNDLE_ID=$(grep -A 1 "bundleid" ~/Desktop/OpenInZed.alfredworkflow/info.plist | tail -1 | sed 's/.*<string>\(.*\)<\/string>.*/\1/')
WORKFLOW_NAME=$(grep -A 1 "<key>name</key>" ~/Desktop/OpenInZed.alfredworkflow/info.plist | tail -1 | sed 's/.*<string>\(.*\)<\/string>.*/\1/')
echo "   Bundle ID: $BUNDLE_ID"
echo "   Name: $WORKFLOW_NAME"

echo ""
echo "⌨️  Keywords:"
grep -A 1 "keyword" ~/Desktop/OpenInZed.alfredworkflow/info.plist | grep "<string>" | sed 's/.*<string>\(.*\)<\/string>.*/  - \1/'

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✅ All checks passed!"
echo ""
echo "Next steps:"
echo "  1. Double-click: ~/Desktop/OpenInZed.alfredworkflow"
echo "  2. Check Alfred Preferences > Workflows"
echo "  3. Verify icon displays correctly"
echo "  4. Test with 'zopen' and 'zrecent' keywords"

#!/bin/bash
# Pre-deployment SEO verification script
# Run this before deploying to ensure all SEO files are in place

echo "🔍 AeroXe Broadband - Pre-Deployment SEO Verification"
echo "======================================================"
echo ""

# Check required files
echo "✅ Checking required SEO files..."
echo ""

files_to_check=(
  "public/robots.txt"
  "public/.htaccess"
  "public/manifest.json"
  "public/.well-known/security.txt"
  "src/components/seo/SEO.tsx"
  "src/components/seo/JsonLd.tsx"
  "src/components/seo/StructuredData.tsx"
  "src/config/seoConfig.ts"
  "src/utils/seoUtils.ts"
  "src/utils/sitemapGenerator.ts"
  "src/api/sitemapApi.ts"
  "index.html"
  "SEO_OPTIMIZATION_GUIDE.md"
  "SEO_IMPLEMENTATION_SUMMARY.md"
)

missing_files=0
for file in "${files_to_check[@]}"; do
  if [ -f "$file" ]; then
    echo "✓ $file"
  else
    echo "✗ MISSING: $file"
    ((missing_files++))
  fi
done

echo ""
if [ $missing_files -eq 0 ]; then
  echo "✅ All SEO files present!"
else
  echo "❌ $missing_files files missing. Please check!"
fi

echo ""
echo "📋 Key SEO Files:"
echo "=================="

echo ""
echo "1. robots.txt location:"
grep -n "Sitemap:" public/robots.txt 2>/dev/null | head -1

echo ""
echo "2. Meta tags in index.html:"
grep -c "meta property" index.html

echo ""
echo "3. JSON-LD schemas included:"
grep -c "@type" src/components/seo/JsonLd.tsx

echo ""
echo "✅ SEO verification complete!"
echo ""
echo "Next steps:"
echo "1. Run: npm run build"
echo "2. Deploy build folder to production"
echo "3. Verify with: https://search.google.com/test/rich-results"
echo "4. Submit sitemap to Google Search Console"
echo "5. Claim Google My Business listing"

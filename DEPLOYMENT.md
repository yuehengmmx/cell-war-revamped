# Cell War - GitHub Pages Deployment Guide

## What's Been Set Up

Your Cell War game is now configured for browser deployment! Here's what was prepared:

### 1. **WASM Build Configuration**
- **Cargo.toml**: Updated with library target and WASM dependencies
- **src/lib.rs**: Created to expose the game as a WASM library
- **.cargo/config.toml**: Configured for WASM-optimized builds

### 2. **Web Hosting Files**
- **index.html**: Main game entry point with responsive styling and WASM loader
- **.nojekyll**: Tells GitHub Pages to serve files as-is
- **.github/workflows/deploy.yml**: Automated build and deployment workflow

### 3. **Project Structure**
- Binary target (desktop): `src/main.rs`
- Library target (web): `src/lib.rs`
- Assets: `assets/` (automatically included in WASM build)

## How to Deploy

### Step 1: Prepare Repository
1. Commit all your game code changes
2. Push to your GitHub repository's `develop` or `main` branch

```bash
git add .
git commit -m "Set up web deployment"
git push origin develop
```

### Step 2: Enable GitHub Pages
1. Go to **Settings** → **Pages**
2. Under "Build and deployment":
   - Change "Source" to "GitHub Actions"
   - Keep the default workflow settings
3. Click Save

### Step 3: Automatic Deployment
- GitHub Actions will automatically build your game when you push to `main` or `develop`
- The workflow builds to WASM and deploys to GitHub Pages
- Your game will be live at: `https://<username>.github.io/cell-war-revamped`

## Monitoring the Build

1. Go to **Actions** tab in your GitHub repository
2. Look for the "Build and Deploy to GitHub Pages" workflow
3. Watch the build progress in real-time
4. Once complete (green checkmark), your game is live!

## Troubleshooting

### Build Fails
- Check the workflow logs in the Actions tab
- Common issue: Permissions - ensure GitHub Pages is enabled in Settings → Pages
- **Solution**: Re-enable GitHub Pages and manually trigger workflow using "Run workflow" button

### Game Doesn't Load
- Open browser DevTools (F12) → Console tab
- Check for error messages
- Common issues:
  - Asset paths: Ensure `assets/` folder is committed to git
  - CORS: GitHub Pages serves from the same origin, so this shouldn't be an issue

### Performance Issues
- Check browser DevTools → Network tab to see load times
- WASM files are typically 10-20 MB (uncompressed)
- GitHub Pages auto-gzips files, so actual download is much smaller

## Local Testing (Optional)

For local WASM testing, you have several options:

### Option 1: Use Simple HTTP Server
```bash
cd /path/to/cell-war-revamped
python -m http.server 8000
```
Then visit: `http://localhost:8000`

**Note**: Local WASM builds can have dependency resolution issues. The GitHub Actions build is recommended.

### Option 2: Keep Desktop Build Locally
```bash
cargo run --release
```
This runs the native desktop version.

## What Players See

When someone visits your game URL, they will see:
- Responsive canvas that scales to their screen
- The Cell War main menu
- Full gameplay in their browser (mouse/keyboard controls work)
- No downloads or installations required

## Game Controls (In Browser)
- **WASD**: Move player
- **Mouse**: Aim/look direction
- **Click/Space**: Shoot
- **SPACE**: Pause/Resume
- **G**: Start Game (from menu)
- **M**: Main Menu
- **ESC**: Exit Game

## Updating Your Game

Every time you:
1. Push code changes to `main` or `develop` branch
2. GitHub Actions automatically:
   - Builds the game as WASM
   - Runs tests (if configured)
   - Deploys to GitHub Pages
3. Your live game updates within 1-5 minutes

## Next Steps

1. ✅ Your code changes are committed and pushed
2. ✅ GitHub Actions workflow is set up
3. ⏳ Enable GitHub Pages in Repository Settings
4. ⏳ Watch the build succeed
5. ⏳ Share your game URL!

## Game URL Format
```
https://YOUR_GITHUB_USERNAME.github.io/cell-war-revamped
```

Replace `YOUR_GITHUB_USERNAME` with your actual GitHub username.

## Additional Notes

- **Assets**: Your `assets/` folder with sprites, audio, and fonts are automatically included
- **Audio**: MP3 and OGG files are supported in WASM builds
- **Mobile**: The game is playable on mobile via browser, but controls are keyboard/mouse based
- **Sharing**: Direct players to the GitHub Pages URL, or add it to your GitHub profile README

Good luck deploying your game! 🎮

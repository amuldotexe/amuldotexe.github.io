# that-in-Rust

A simple, elegant landing page for my Rust learning journey.

## 🌐 Live Site

Visit the live site at [https://amuldotexe.github.io](https://amuldotexe.github.io)

## 🏗️ Project Structure

```
.
├── index.html          # Main landing page
├── txtRef/            # Reference documents
│   ├── ref01currentState.txt
│   ├── ref02newPRD.txt
│   └── ref03archL1.txt
└── .github/workflows/  # GitHub Actions workflows
    └── deploy.yml     # Deployment configuration
```

## 🚀 Deployment

This site is deployed using GitHub Pages with GitHub Actions. The deployment process works as follows:

1. Push changes to the `main` branch
2. GitHub Actions workflow automatically:
   - Creates a clean deployment directory
   - Copies only the necessary static files
   - Uploads the artifact
   - Deploys to GitHub Pages

### Deployment Configuration

The deployment is handled by `.github/workflows/deploy.yml` which uses official GitHub Pages actions:
- `actions/configure-pages@v3`: Sets up GitHub Pages
- `actions/upload-pages-artifact@v2`: Bundles the site
- `actions/deploy-pages@v2`: Deploys to Pages

No build step is required as this is a pure static site.

## 🛠️ Development

To work on this site locally:

1. Clone the repository:
   ```bash
   git clone https://github.com/amuldotexe/amuldotexe.github.io.git
   cd amuldotexe.github.io
   ```

2. Make your changes to the HTML/CSS in `index.html`

3. Test locally by opening `index.html` in a browser

4. Push changes to GitHub:
   ```bash
   git add .
   git commit -m "Your commit message"
   git push origin main
   ```

The site will automatically deploy after pushing to main.

## 📝 License

MIT License - feel free to use this code as you wish!

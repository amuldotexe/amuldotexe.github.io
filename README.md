# amuldotexe.github.io

**algorithmic-intuition** -- interactive algorithm simulations with a terminal aesthetic. Step through algorithms visually, one click at a time.

## Live Site

[https://amuldotexe.github.io](https://amuldotexe.github.io)

## Algorithms

| Algorithm | Status |
|-----------|--------|
| Binary Search | done |
| (more coming) | -- |

## Project Structure

```
.
├── index.html              # Landing page
├── css/
│   └── shared.css          # Shared terminal theme
├── js/
│   └── binary-search.js    # Binary search simulation logic
├── algorithms/
│   └── binary-search.html  # Binary search simulation page
└── .github/workflows/
    └── deploy.yml          # GitHub Pages deployment
```

## Adding a New Algorithm

1. Create `algorithms/<name>.html` using the existing page as a template
2. Create `js/<name>.js` with step precomputation and rendering logic
3. Add a card link in `index.html` under the algorithms section
4. Add copy lines in `.github/workflows/deploy.yml`

## Development

Open any HTML file directly in a browser. No build tools needed.

## License

MIT

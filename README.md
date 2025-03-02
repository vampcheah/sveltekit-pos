# Svelte POS System

A modern, responsive Point of Sale (POS) system built with Svelte, Tailwind CSS, and shadcn-svelte components.

## Features

- **Intuitive Interface**: Clean, user-friendly design optimized for both desktop and touch devices
- **Real-time Updates**: Reactive UI that instantly reflects changes to cart and inventory
- **Product Management**: Easily add, edit, and categorize products
- **Transaction Processing**: Smooth checkout flow with multiple payment options
- **Weight-based Products**: Support for products sold by weight with custom input dialog
- **Responsive Design**: Works seamlessly across all device sizes

## Tech Stack

- **Frontend**: [Svelte V5](https://svelte.dev/) with the new runes system
- **Styling**: [Tailwind CSS](https://tailwindcss.com/)
- **UI Components**: [shadcn-svelte](https://www.shadcn-svelte.com/)

## Getting Started

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/svelte-pos.git
   cd svelte-pos
   ```

2. Install dependencies:

   ```bash
   npm install
   ```

3. Start the development server:

   ```bash
   npm run dev
   ```

4. Open your browser and navigate to `http://localhost:5173`

## Project Structure

```bash
svelte-pos/
├── src/
│ ├── lib/
│ │ ├── components/ # Reusable UI components
│ │ ├── state.svelte.ts # Global state management
│ │ └── ...
│ ├── routes/
│ │ ├── page-pos/ # Main POS interface
│ │ │ ├── +page.svelte # POS page component
│ │ │ ├── ProductItem.svelte
│ │ │ ├── CartItem.svelte
│ │ │ └── ...
│ │ └── ...
│ └── ...
└── ...
```

## Usage

### Adding Products to Cart

Click on product items in the product grid to add them to the shopping cart. For weight-based products, a dialog will appear to input the weight.

### Checkout Process

1. Add all desired products to the cart
2. Review the cart items and total
3. Select payment method
4. Complete the transaction

## Customization

### Adding New Products

Products can be added through the admin interface or by modifying the product data source.

### Theming

The application uses Tailwind CSS for styling. Customize the look and feel by modifying the `tailwind.config.js` file.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- [Svelte](https://svelte.dev/) - The frontend framework
- [Tailwind CSS](https://tailwindcss.com/) - For styling
- [shadcn-svelte](https://next.shadcn-svelte.com/docs) - For UI components

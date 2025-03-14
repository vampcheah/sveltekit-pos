# Sveltekit POS

> This system is not recommended for full commercial use. Further customization and development are encouraged to enhance the overall user experience.

Sveltekit POS (vECO) is a modern, responsive Point of Sale solution built with Sveltekit, Tailwind CSS, and shadcn-svelte components. Designed for modern browsers, it is fully functional offline and can be installed as a Progressive Web App (PWA), final goal. The system utilizes IndexedDB and LocalStorage for local data management, without relying on external databases.

Explores real-world applications of SvelteKit, providing a flexible design framework that is easy to extend and adapt for additional development.

## Features

- **Intuitive Interface**: Clean, user-friendly design optimized for both desktop and touch devices
- **Real-time Updates**: Reactive UI that instantly reflects changes to cart and inventory
- **Product Management**: Easily add, edit, and categorize products
- **Transaction Processing**: Smooth checkout flow with multiple payment options
- **Weight-based Products**: Support for products sold by weight with custom input dialog
- **Responsive Design**: Works seamlessly across all device sizes

## Tech Stack

- **Frontend**: [Sveltekit/Svelte V5](https://svelte.dev/) with the new runes system
- **Styling**: [Tailwind CSS](https://tailwindcss.com/)
- **UI Components**: [shadcn-svelte](https://www.shadcn-svelte.com/)

## Limitations

1. Unsupported browsers:
   Although most modern browsers support IndexedDB, older browsers (such as IE 10 and below) or certain mobile browsers may not support it.

2. Limited storage capacity:
   IndexedDB storage space is not unlimited and varies depending on the browser and device. Users might encounter issues where data cannot be written due to space limitations.

3. Automatic cache clearing:
   Browsers might automatically clear IndexedDB data if the storage reaches a certain size, especially if the data has not been used in a while, potentially causing data loss.

4. User clearing browser data:
   Users can manually clear their browser data or select "Clear Site Data," which would remove all IndexedDB data.

5. Performance degradation with large data sets:
   Query performance in IndexedDB may degrade when handling large amounts of data, especially if indexes are not properly designed.

6. Slower read/write operations:
   The asynchronous read/write operations of IndexedDB can be slower compared to in-memory operations. Frequent reads and writes of large data may negatively affect user experience.

## Getting Started

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/vampcheah/sveltekit-pos.git
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

4. Open browser and navigate to `http://localhost:5173`

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

## ToDo List

### Cryptocurrency Payment Module

Add support for cryptocurrency payments, giving users more flexibility by allowing them to make transactions using various digital currencies.

### Basic User Dashboard

A simple yet functional user dashboard will be added, providing users with essential account management features, including order history, profile settings, and more.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- [Svelte/Sveltekit](https://svelte.dev/) - The frontend framework
- [Tailwind CSS](https://tailwindcss.com/) - For styling
- [shadcn-svelte](https://next.shadcn-svelte.com/docs) - For UI components

## Donations

- Donate to support: 0xbCF24200CeaB882c3b29d060A5b77217a8dd83Dc

## Version

- current-dev
  - IndexedDB to product and category
  - LICENSE-MIT
- 0.0.7
  - Implementing currency to display price
  - Tax supported
  - Discount supported
  - Fixed: cannot checkout after refresh the page or load from local saved cart
  - Customer checkout cart (Screen UI)
  - Synchronization between customer screen and cashier
  - Foldering: POS Cashier / Display
- 0.0.6
  - Implementing localstorage to host persistance storage
  - Multi-language supported on UI
  - Enhancement on cart management with mobile support
  - Improvement on Dialog style
- 0.0.5
  - Replace alert with toast
  - Payment dialog after checkout
- 0.0.4
  - Centralized cart state
  - Basic data management
- 0.0.3
  - Mobile friendly
  - Responsive design
  - Added mobile bottom navigation
- 0.0.2
  - IPad compatibility
  - Auto detect screen height
  - Scrolling product grid
- 0.0.1
  - Initial setup and basic functionality

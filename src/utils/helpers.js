/**
 * Format a WhatsApp number for URL usage
 * Removes any non-digit characters
 *
 * @param {string} number - WhatsApp number
 * @returns {string} Cleaned number
 */
export function formatWhatsAppNumber(number) {
  return number.replace(/\D/g, '');
}

/**
 * Encode a message for WhatsApp URL
 *
 * @param {string} message - Message to encode
 * @returns {string} URL-encoded message
 */
export function encodeWhatsAppMessage(message) {
  return encodeURIComponent(message.trim());
}

/**
 * Generate WhatsApp URL with pre-filled message
 *
 * @param {string} number - WhatsApp number
 * @param {string} message - Message content
 * @returns {string} Full WhatsApp URL
 */
export function getWhatsAppUrl(number, message) {
  const cleaned = formatWhatsAppNumber(number);
  const encoded = encodeWhatsAppMessage(message);
  return `https://wa.me/${cleaned}?text=${encoded}`;
}

/**
 * Generate contact form WhatsApp message
 *
 * @param {Object} data - Form data
 * @returns {string} Formatted WhatsApp message
 */
export function generateContactMessage(data) {
  const { name, mobile, email, address, message } = data;
  return [
    'Hello,',
    '',
    'I am interested in your Internet Service.',
    '',
    `Name: ${name}`,
    `Mobile: ${mobile}`,
    `Email: ${email}`,
    `Address: ${address}`,
    `Message: ${message}`,
    '',
    'Please contact me.',
  ].join('\n');
}

import { useState } from 'react';
import { motion } from 'framer-motion';
import { FiSend, FiCheck, FiAlertCircle, FiUser, FiPhone, FiMail, FiMapPin, FiMessageSquare } from 'react-icons/fi';
import { SITE_CONFIG } from '../config/site';
import { useIntersectionObserver } from '../hooks/useIntersectionObserver';
import { fadeUp, staggerContainer, staggerItem } from '../utils/animations';
import { getWhatsAppUrl, generateContactMessage } from '../utils/helpers';

/**
 * Contact Form Component
 * Glassmorphism form that constructs a WhatsApp message on submission.
 * No backend required — opens WhatsApp with pre-filled form data.
 */
export default function ContactForm() {
  const [ref, isVisible] = useIntersectionObserver({ threshold: 0.1 });
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [isSuccess, setIsSuccess] = useState(false);
  const [errors, setErrors] = useState({});

  const [formData, setFormData] = useState({
    name: '',
    mobile: '',
    email: '',
    address: '',
    message: '',
    agree: false,
  });

  const validateForm = () => {
    const newErrors = {};

    if (!formData.name.trim()) {
      newErrors.name = 'Full name is required';
    }

    if (!formData.mobile.trim()) {
      newErrors.mobile = 'Mobile number is required';
    } else if (!/^(\+91|0)?[6-9]\d{9}$/.test(formData.mobile.trim())) {
      newErrors.mobile = 'Please enter a valid 10-digit Indian mobile number';
    }

    if (!formData.email.trim()) {
      newErrors.email = 'Email is required';
    } else if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(formData.email.trim())) {
      newErrors.email = 'Please enter a valid email';
    }

    if (!formData.address.trim()) {
      newErrors.address = 'Address is required';
    }

    if (!formData.message.trim()) {
      newErrors.message = 'Message is required';
    }

    if (!formData.agree) {
      newErrors.agree = 'You must agree to be contacted';
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleChange = (e) => {
    const { name, value, type, checked } = e.target;
    setFormData((prev) => ({
      ...prev,
      [name]: type === 'checkbox' ? checked : value,
    }));
    // Clear error on change
    if (errors[name]) {
      setErrors((prev) => ({ ...prev, [name]: undefined }));
    }
  };

  const handleSubmit = async (e) => {
    e.preventDefault();

    if (!validateForm()) return;

    setIsSubmitting(true);

    // Simulate loading before redirect
    await new Promise((resolve) => setTimeout(resolve, 800));

    try {
      const message = generateContactMessage(formData);
      const whatsappUrl = getWhatsAppUrl(SITE_CONFIG.whatsapp, message);

      // Open WhatsApp in new tab
      window.open(whatsappUrl, '_blank', 'noopener,noreferrer');

      setIsSuccess(true);

      // Reset form after success
      setFormData({
        name: '',
        mobile: '',
        email: '',
        address: '',
        message: '',
        agree: false,
      });

      // Reset success message after 3 seconds
      setTimeout(() => setIsSuccess(false), 3000);
    } catch (error) {
      setErrors({ submit: 'Something went wrong. Please try again.' });
    } finally {
      setIsSubmitting(false);
    }
  };

  const inputClasses = (fieldName) =>
    `w-full px-4 py-3.5 bg-white/[0.04] border ${
      errors[fieldName]
        ? 'border-red-500/50 focus:border-red-400'
        : 'border-white/[0.08] focus:border-accent-500/50'
    } rounded-xl text-white placeholder:text-dark-500 focus:outline-none focus:ring-1 focus:ring-accent-500/30 transition-all duration-300 text-sm`;

  const labelClasses = 'block text-sm font-medium text-dark-300 mb-2';

  return (
    <section id="contact" className="relative py-20 lg:py-32 overflow-hidden" aria-label="Contact us">
      <div className="absolute inset-0 bg-gradient-to-b from-dark-950 via-dark-900 to-dark-950" />

      <div className="relative max-w-7xl mx-auto px-4 sm:px-6 lg:px-8" ref={ref}>
        {/* Section header */}
        <motion.div
          variants={fadeUp}
          initial="hidden"
          animate={isVisible ? 'visible' : 'hidden'}
          className="text-center max-w-3xl mx-auto mb-16 lg:mb-20"
        >
          <span className="text-xs lg:text-sm font-semibold text-accent-400 tracking-[0.2em] uppercase mb-4 block">
            Contact
          </span>
          <h2 className="text-3xl sm:text-4xl lg:text-5xl font-bold text-white mb-6">
            Let's{' '}
            <span className="bg-gradient-to-r from-accent-400 to-primary-400 bg-clip-text text-transparent">
              Get Connected
            </span>
          </h2>
          <p className="text-base lg:text-lg text-dark-300 leading-relaxed">
            Fill in your details and we'll connect with you on WhatsApp instantly.
          </p>
        </motion.div>

        {/* Form */}
        <motion.div
          variants={fadeUp}
          initial="hidden"
          animate={isVisible ? 'visible' : 'hidden'}
          className="max-w-2xl mx-auto"
        >
          <div className="relative">
            {/* Glassmorphism card */}
            <div className="relative p-6 lg:p-10 rounded-3xl bg-gradient-to-b from-white/[0.04] to-white/[0.01] border border-white/[0.08] backdrop-blur-xl shadow-2xl shadow-dark-900/50">
              {/* Glow effect */}
              <div className="absolute -top-20 -right-20 w-40 h-40 bg-accent-500/10 rounded-full blur-3xl pointer-events-none" />
              <div className="absolute -bottom-20 -left-20 w-40 h-40 bg-primary-500/10 rounded-full blur-3xl pointer-events-none" />

              <form onSubmit={handleSubmit} className="relative space-y-5" noValidate>
                {/* Success message */}
                {isSuccess && (
                  <motion.div
                    initial={{ opacity: 0, y: -10 }}
                    animate={{ opacity: 1, y: 0 }}
                    className="flex items-center gap-3 p-4 rounded-xl bg-accent-500/10 border border-accent-500/20 text-accent-300"
                  >
                    <FiCheck className="w-5 h-5 flex-shrink-0" />
                    <span className="text-sm font-medium">
                      WhatsApp opened successfully! We'll connect with you shortly.
                    </span>
                  </motion.div>
                )}

                {/* Error message */}
                {errors.submit && (
                  <div className="flex items-center gap-3 p-4 rounded-xl bg-red-500/10 border border-red-500/20 text-red-300">
                    <FiAlertCircle className="w-5 h-5 flex-shrink-0" />
                    <span className="text-sm font-medium">{errors.submit}</span>
                  </div>
                )}

                <div className="grid sm:grid-cols-2 gap-5">
                  {/* Name */}
                  <div>
                    <label htmlFor="name" className={labelClasses}>
                      Full Name *
                    </label>
                    <div className="relative">
                      <FiUser className="absolute left-3.5 top-1/2 -translate-y-1/2 w-4 h-4 text-dark-500" />
                      <input
                        id="name"
                        name="name"
                        type="text"
                        value={formData.name}
                        onChange={handleChange}
                        className={`${inputClasses('name')} pl-11`}
                        placeholder="Rahul Sharma"
                        aria-invalid={!!errors.name}
                        aria-describedby={errors.name ? 'name-error' : undefined}
                      />
                    </div>
                    {errors.name && (
                      <p id="name-error" className="mt-1.5 text-xs text-red-400">
                        {errors.name}
                      </p>
                    )}
                  </div>

                  {/* Mobile */}
                  <div>
                    <label htmlFor="mobile" className={labelClasses}>
                      Mobile Number *
                    </label>
                    <div className="relative">
                      <FiPhone className="absolute left-3.5 top-1/2 -translate-y-1/2 w-4 h-4 text-dark-500" />
                      <input
                        id="mobile"
                        name="mobile"
                        type="tel"
                        value={formData.mobile}
                        onChange={handleChange}
                        className={`${inputClasses('mobile')} pl-11`}
                        placeholder="9999999999"
                        aria-invalid={!!errors.mobile}
                        aria-describedby={errors.mobile ? 'mobile-error' : undefined}
                      />
                    </div>
                    {errors.mobile && (
                      <p id="mobile-error" className="mt-1.5 text-xs text-red-400">
                        {errors.mobile}
                      </p>
                    )}
                  </div>
                </div>

                <div className="grid sm:grid-cols-2 gap-5">
                  {/* Email */}
                  <div>
                    <label htmlFor="email" className={labelClasses}>
                      Email Address *
                    </label>
                    <div className="relative">
                      <FiMail className="absolute left-3.5 top-1/2 -translate-y-1/2 w-4 h-4 text-dark-500" />
                      <input
                        id="email"
                        name="email"
                        type="email"
                        value={formData.email}
                        onChange={handleChange}
                        className={`${inputClasses('email')} pl-11`}
                        placeholder="rahul@email.com"
                        aria-invalid={!!errors.email}
                        aria-describedby={errors.email ? 'email-error' : undefined}
                      />
                    </div>
                    {errors.email && (
                      <p id="email-error" className="mt-1.5 text-xs text-red-400">
                        {errors.email}
                      </p>
                    )}
                  </div>

                  {/* Address */}
                  <div>
                    <label htmlFor="address" className={labelClasses}>
                      Address *
                    </label>
                    <div className="relative">
                      <FiMapPin className="absolute left-3.5 top-1/2 -translate-y-1/2 w-4 h-4 text-dark-500" />
                      <input
                        id="address"
                        name="address"
                        type="text"
                        value={formData.address}
                        onChange={handleChange}
                        className={`${inputClasses('address')} pl-11`}
                        placeholder="Your full address"
                        aria-invalid={!!errors.address}
                        aria-describedby={errors.address ? 'address-error' : undefined}
                      />
                    </div>
                    {errors.address && (
                      <p id="address-error" className="mt-1.5 text-xs text-red-400">
                        {errors.address}
                      </p>
                    )}
                  </div>
                </div>

                {/* Message */}
                <div>
                  <label htmlFor="message" className={labelClasses}>
                    Message *
                  </label>
                  <div className="relative">
                    <FiMessageSquare className="absolute left-3.5 top-3.5 w-4 h-4 text-dark-500" />
                    <textarea
                      id="message"
                      name="message"
                      value={formData.message}
                      onChange={handleChange}
                      rows={4}
                      className={`${inputClasses('message')} pl-11 resize-none`}
                      placeholder="Tell us about your internet needs..."
                      aria-invalid={!!errors.message}
                      aria-describedby={errors.message ? 'message-error' : undefined}
                    />
                  </div>
                  {errors.message && (
                    <p id="message-error" className="mt-1.5 text-xs text-red-400">
                      {errors.message}
                    </p>
                  )}
                </div>

                {/* Checkbox */}
                <div>
                  <label className="flex items-start gap-3 cursor-pointer group">
                    <input
                      type="checkbox"
                      name="agree"
                      checked={formData.agree}
                      onChange={handleChange}
                      className="mt-0.5 w-4 h-4 rounded border-dark-500 bg-white/5 text-accent-500 focus:ring-accent-500 focus:ring-offset-0 cursor-pointer"
                      aria-invalid={!!errors.agree}
                      aria-describedby={errors.agree ? 'agree-error' : undefined}
                    />
                    <span className="text-sm text-dark-400 group-hover:text-dark-300 transition-colors">
                      I agree to be contacted regarding my inquiry.
                      <span className="text-red-400"> *</span>
                    </span>
                  </label>
                  {errors.agree && (
                    <p id="agree-error" className="mt-1.5 text-xs text-red-400 ml-7">
                      {errors.agree}
                    </p>
                  )}
                </div>

                {/* Submit button */}
                <motion.button
                  type="submit"
                  disabled={isSubmitting}
                  className="w-full relative px-8 py-4 text-base font-semibold text-white rounded-2xl overflow-hidden group focus:outline-none focus:ring-2 focus:ring-accent-400 focus:ring-offset-2 focus:ring-offset-dark-900 disabled:opacity-70 disabled:cursor-not-allowed"
                  whileHover={!isSubmitting ? { scale: 1.01 } : {}}
                  whileTap={!isSubmitting ? { scale: 0.99 } : {}}
                >
                  <span className="absolute inset-0 bg-gradient-to-r from-primary-600 to-accent-600 group-hover:from-primary-500 group-hover:to-accent-500 transition-all duration-300" />
                  <span className="absolute inset-0 bg-gradient-to-r from-accent-500 to-primary-500 opacity-0 group-hover:opacity-100 transition-opacity duration-300" />
                  <span className="relative z-10 flex items-center justify-center gap-2">
                    {isSubmitting ? (
                      <>
                        <motion.div
                          animate={{ rotate: 360 }}
                          transition={{ duration: 1, repeat: Infinity, ease: 'linear' }}
                          className="w-5 h-5 border-2 border-white/30 border-t-white rounded-full"
                        />
                        Sending...
                      </>
                    ) : (
                      <>
                        Send via WhatsApp
                        <FiSend className="w-4 h-4 group-hover:translate-x-1 transition-transform duration-300" />
                      </>
                    )}
                  </span>
                </motion.button>
              </form>
            </div>
          </div>
        </motion.div>
      </div>
    </section>
  );
}

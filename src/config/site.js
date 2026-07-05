/**
 * Site Configuration
 *
 * Update these values to match your ISP business.
 * The WhatsApp number is used by the contact form.
 */

export const SITE_CONFIG = {
  // WhatsApp business number (without + sign)
  whatsapp: "917770033326",

  // Google Maps location
  location: {
    mapUrl: "https://maps.app.goo.gl/gZXzQKuHh6rcp55Q8?g_st=aw",
    city: "Jalgaon",
    state: "Maharashtra",
    country: "India",
  },

  // Company info
  company: {
    name: "AeroXe Broadband",
    tagline: "Lightning Fast Fiber Internet",
    description:
      "Premium fiber optic internet services in Jalgaon, Maharashtra. Reliable, fast, and affordable connectivity solutions for home and business.",
    phone: "+91 77700 33326",
    email: "support@aeroXe.in",
    address: "Jalgaon, Maharashtra, India",
  },

  // Social media links
  social: {
    facebook: "https://facebook.com/aeroXe",
    twitter: "https://twitter.com/aeroXe",
    instagram: "https://instagram.com/aeroXe",
    linkedin: "https://linkedin.com/company/aeroXe",
    youtube: "https://youtube.com/@aeroXe",
  },

  // Navigation links
  navLinks: [
    { label: "Home", href: "#home" },
    { label: "Plans", href: "#plans" },
    { label: "Features", href: "#features" },
    { label: "Coverage", href: "#coverage" },
    { label: "About", href: "#about" },
    { label: "FAQ", href: "#faq" },
    { label: "Contact", href: "#contact" },
  ],

  // Pricing plans
  plans: [
    {
      speed: "50 Mbps",
      tag: "Basic",
      popular: false,
      durations: {
        1: { price: 400, label: "1 Month" },
        3: { price: 1150, label: "3 Months", savings: "Save ₹50" },
        6: { price: 2250, label: "6 Months", savings: "Save ₹150" },
        12: { price: 4300, label: "12 Months", savings: "Save ₹500" },
      },
      features: [
        "Unlimited Data",
        "Free Installation",
        "24/7 Support",
        "99.99% Uptime",
      ],
    },
    {
      speed: "100 Mbps",
      tag: "Standard",
      popular: true,
      durations: {
        1: { price: 600, label: "1 Month" },
        3: { price: 1700, label: "3 Months", savings: "Save ₹100" },
        6: { price: 3350, label: "6 Months", savings: "Save ₹250" },
        12: { price: 6400, label: "12 Months", savings: "Save ₹800" },
      },
      features: [
        "Unlimited Data",
        "Free Installation",
        "24/7 Support",
        "99.99% Uptime",
        "Dual Band WiFi Router Free*",
      ],
    },
    {
      speed: "150 Mbps",
      tag: "Premium",
      popular: false,
      durations: {
        1: { price: 800, label: "1 Month" },
        3: { price: 2300, label: "3 Months", savings: "Save ₹100" },
        6: { price: 4550, label: "6 Months", savings: "Save ₹250" },
        12: { price: 8700, label: "12 Months", savings: "Save ₹900" },
      },
      features: [
        "Unlimited Data",
        "Free Installation",
        "24/7 Support",
        "99.99% Uptime",
        "Dual Band WiFi Router Free*",
        "Priority Support",
      ],
    },
    {
      speed: "200 Mbps",
      tag: "Pro",
      popular: false,
      durations: {
        1: { price: 1000, label: "1 Month" },
        3: { price: 2850, label: "3 Months", savings: "Save ₹150" },
        6: { price: 5650, label: "6 Months", savings: "Save ₹350" },
        12: { price: 10800, label: "12 Months", savings: "Save ₹1200" },
      },
      features: [
        "Unlimited Data",
        "Free Installation",
        "24/7 Support",
        "99.99% Uptime",
        "Dual Band WiFi Router Free*",
        "Priority Support",
        "Static IP Option",
      ],
    },
    {
      speed: "300 Mbps",
      tag: "Ultimate",
      popular: false,
      durations: {
        1: { price: 1300, label: "1 Month" },
        3: { price: 3700, label: "3 Months", savings: "Save ₹200" },
        6: { price: 7350, label: "6 Months", savings: "Save ₹450" },
        12: { price: 14000, label: "12 Months", savings: "Save ₹1600" },
      },
      features: [
        "Unlimited Data",
        "Free Installation",
        "24/7 Support",
        "99.99% Uptime",
        "Dual Band WiFi Router Free*",
        "Priority Support",
        "Static IP Option",
        "Business Grade",
      ],
    },
  ],

  // Features
  features: [
    {
      icon: "Zap",
      title: "High Speed Fiber",
      description:
        "Blazing-fast internet with speeds up to 300 Mbps over fiber optic. Perfect for streaming, gaming, video calls, and remote work.",
    },
    {
      icon: "Infinity",
      title: "Unlimited Internet",
      description:
        "No data caps, no FUP limits, no throttling. Enjoy truly unlimited internet access on all your devices, all day every day.",
    },
    {
      icon: "Monitor",
      title: "Free WiFi Router",
      description:
        "Get a free Dual Band WiFi router with 12-month plans. Enjoy strong, stable WiFi coverage throughout your home or office.",
    },
    {
      icon: "Headphones",
      title: "24/7 Support",
      description:
        "Round-the-clock customer support via phone and WhatsApp. We're always here when you need us — day or night.",
    },
    {
      icon: "Zap",
      title: "Free Installation",
      description:
        "Professional free installation by our expert technicians. Limited period offer — get connected without any setup fees.",
    },
    {
      icon: "Shield",
      title: "99.99% Uptime",
      description:
        "Enterprise-grade reliability with 99.99% network uptime. Our robust fiber infrastructure ensures you stay connected always.",
    },
    {
      icon: "BadgePercent",
      title: "Affordable Plans",
      description:
        "Premium internet at the most competitive prices in Jalgaon. Plans starting from just ₹400/month with flexible durations.",
    },
    {
      icon: "Globe",
      title: "Local Service",
      description:
        "Proudly serving Jalgaon with personalized local support. We understand our community's needs and deliver tailored solutions.",
    },
  ],

  // Why choose us
  whyChooseUs: [
    {
      title: "Free Installation",
      description: "Limited period offer — get connected absolutely free. No installation or setup charges.",
    },
    {
      title: "Free Dual Band Router",
      description: "12-month plans include a complimentary Dual Band WiFi router for seamless connectivity.",
    },
    {
      title: "99.99% Uptime",
      description: "Reliable fiber optic network with 99.99% uptime guarantee for uninterrupted internet.",
    },
    {
      title: "Truly Unlimited Data",
      description: "No FUP limits, no hidden caps. Stream, download, and browse as much as you want.",
    },
    {
      title: "Affordable Pricing",
      description: "Competitive plans starting at just ₹400/month. Save more with our longer duration plans.",
    },
    {
      title: "24/7 Local Support",
      description: "Expert support team based in Jalgaon, available anytime via phone or WhatsApp.",
    },
    {
      title: "Fiber Backbone",
      description: "Modern fiber optic infrastructure delivering consistent high-speed connectivity.",
    },
  ],

  // Installation steps
  installationSteps: [
    {
      step: 1,
      title: "Contact Us",
      description: "Call or WhatsApp us at 77700 33326, or fill out our contact form to get started.",
    },
    {
      step: 2,
      title: "Verification",
      description: "We verify your address in Jalgaon and surrounding areas to confirm coverage availability.",
    },
    {
      step: 3,
      title: "Free Installation",
      description: "Our expert technicians visit your premises and install the fiber connection at no cost.",
    },
    {
      step: 4,
      title: "Activation",
      description: "Your connection is activated instantly. Start enjoying high-speed fiber internet immediately.",
    },
  ],

  // Testimonials
  testimonials: [
    {
      name: "Rahul Sharma",
      role: "Remote Professional, Jalgaon",
      feedback:
        "AeroXe Broadband has been a game-changer for my work-from-home setup. The 100 Mbps plan is rock solid and I've never faced a single disconnection in 3 months!",
      rating: 5,
    },
    {
      name: "Priya Patil",
      role: "College Student",
      feedback:
        "Finally, reliable internet in Jalgaon! No more buffering during online classes. The free WiFi router works great throughout our home. Highly recommended!",
      rating: 5,
    },
    {
      name: "Amit Deshmukh",
      role: "Small Business Owner",
      feedback:
        "Our entire business runs on AeroXe's 200 Mbps connection. Video calls are crystal clear, and their support team in Jalgaon is incredibly responsive.",
      rating: 5,
    },
    {
      name: "Sneha Joshi",
      role: "Streaming Enthusiast",
      feedback:
        "4K streaming on Netflix and Prime Video without any buffering — even with multiple devices connected. AeroXe is the best ISP in Jalgaon, hands down.",
      rating: 5,
    },
    {
      name: "Vikram Singh",
      role: "Gamer, Jalgaon",
      feedback:
        "Low ping, consistent speeds, and zero lag in competitive games. The 150 Mbps plan is perfect for gaming. Plus, the free installation was quick and clean!",
      rating: 5,
    },
  ],

  // FAQ data
  faqs: [
    {
      question: "How quickly can I get internet installed in Jalgaon?",
      answer:
        "We typically install within 24-48 hours of your request. In many areas of Jalgaon, same-day installation is available. Plus, installation is absolutely free for a limited period!",
    },
    {
      question: "What internet speeds do you offer?",
      answer:
        "We offer fiber optic plans from 50 Mbps to 300 Mbps. All plans provide symmetrical speeds and truly unlimited data with no FUP limits.",
    },
    {
      question: "How do I contact customer support?",
      answer:
        "Our support team is available 24/7 via phone at 77700 33326, WhatsApp at the same number, or through our website contact form. We're based in Jalgaon and always happy to help.",
    },
    {
      question: "What internet plans do you have?",
      answer:
        "We have 5 speed tiers: 50 Mbps (₹400/mo), 100 Mbps (₹600/mo), 150 Mbps (₹800/mo), 200 Mbps (₹1,000/mo), and 300 Mbps (₹1,300/mo). Each comes in 1-month, 3-month, 6-month, and 12-month durations with increasing savings.",
    },
    {
      question: "Is AeroXe available in my area?",
      answer:
        "We are currently serving Jalgaon with plans to expand to Bhusawal, Mumbai, Navi Mumbai, and Barhanpur soon. Contact us to check availability at your specific location.",
    },
    {
      question: "Do I get a free WiFi router?",
      answer:
        "Yes! All 12-month plans include a complimentary Dual Band WiFi router. This ensures strong, stable WiFi coverage throughout your home or office.",
    },
    {
      question: "Is installation really free?",
      answer:
        "Yes, installation is completely free for a limited period. Our expert technicians will visit your premises and set up everything at no cost to you.",
    },
    {
      question: "What is fiber optic internet?",
      answer:
        "Fiber optic internet uses light signals through glass cables to deliver data at lightning speeds. It's faster, more reliable, and has lower latency than traditional copper connections. Perfect for streaming, gaming, and remote work.",
    },
    {
      question: "What payment methods do you accept?",
      answer:
        "We accept UPI (Google Pay, PhonePe, Paytm), net banking, and cash payments. You can choose monthly, quarterly, half-yearly, or annual billing cycles.",
    },
    {
      question: "Do you offer business internet plans?",
      answer:
        "Yes! Our 200 Mbps and 300 Mbps plans are ideal for businesses, offering priority support, static IP options, and guaranteed uptime for your operations.",
    },
  ],

  // Coverage areas
  coverageAreas: [
    { name: "Jalgaon", status: "active", type: "city" },
    { name: "Bhusawal", status: "coming-soon", type: "city" },
    { name: "Mumbai", status: "coming-soon", type: "city" },
    { name: "Navi Mumbai", status: "coming-soon", type: "city" },
    { name: "Barhanpur", status: "coming-soon", type: "city" },
    // Areas within Jalgaon
    { name: "Jalgaon City Center", status: "active", type: "area" },
    { name: "Shirpur Road", status: "active", type: "area" },
    { name: "Mahabal", status: "active", type: "area" },
    { name: "Ravivar Peth", status: "active", type: "area" },
    { name: "Navipeth", status: "active", type: "area" },
    { name: "Nagar Parishad", status: "active", type: "area" },
    { name: "Bhusawal Road", status: "active", type: "area" },
    { name: "MIDC Area", status: "active", type: "area" },
    { name: "Railway Station Area", status: "active", type: "area" },
  ],
};

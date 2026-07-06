import { motion } from 'framer-motion';
import { Users, Quote } from 'lucide-react';
import { SITE_CONFIG } from '../../config/site';
import SEO from '../../components/seo/SEO';

const initials = (name: string) =>
  name.split(' ').map((n) => n[0]).join('');

const bgColors = [
  'from-accent-400 to-primary-500',
  'from-primary-400 to-accent-500',
  'from-accent-500 to-cyan-400',
  'from-cyan-400 to-primary-400',
  'from-primary-500 to-accent-400',
  'from-accent-300 to-primary-500',
];

export default function TeamPage() {
  return (
    <>
      <SEO
        title="Meet the Team — AeroXe Broadband"
        description="Meet the founders behind AeroXe Broadband. A dedicated team bringing premium fiber internet to Jalgaon with expertise in operations, networking, and software."
        path="/team"
      />
    <div className="min-h-screen pt-24 pb-16 bg-dark-950">
      <div className="absolute inset-0 bg-[radial-gradient(ellipse_at_top,rgba(6,182,212,0.06),transparent_50%)]" />

      <div className="relative max-w-[92%] 2xl:max-w-[90rem] mx-auto px-4 sm:px-6">
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          className="text-center mb-16"
        >
          <motion.span
            initial={{ opacity: 0, scale: 0.9 }}
            animate={{ opacity: 1, scale: 1 }}
            transition={{ duration: 0.4 }}
            className="inline-flex items-center gap-2 px-4 py-1.5 rounded-full bg-accent-400/10 border border-accent-400/20 text-accent-300 text-sm font-medium mb-5"
          >
            <Users className="w-4 h-4" />
            Our Team
          </motion.span>
          <h1 className="text-3xl sm:text-4xl lg:text-5xl font-bold text-white">
            Meet the{' '}
            <span className="bg-gradient-to-r from-accent-300 to-primary-400 bg-clip-text text-transparent">
              People Behind AeroXe
            </span>
          </h1>
          <p className="mt-4 text-lg text-dark-400 max-w-3xl mx-auto">
            A passionate team of local experts dedicated to connecting {SITE_CONFIG.location.city} with the fastest fiber internet.
          </p>
        </motion.div>

        <div className="grid sm:grid-cols-2 lg:grid-cols-3 gap-6">
          {SITE_CONFIG.team.map((member, i) => (
            <motion.div
              key={member.name}
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ delay: 0.1 + i * 0.06 }}
              className="group rounded-2xl border border-white/[0.06] bg-white/[0.02] p-6 hover:border-accent-400/20 hover:bg-accent-400/[0.02] transition-all duration-300"
            >
              <div className="flex items-center gap-4 mb-4">
                {member.photo ? (
                  <img
                    src={member.photo}
                    alt={member.name}
                    className="w-16 h-16 rounded-full object-cover"
                  />
                ) : (
                  <div className={`w-16 h-16 rounded-full bg-gradient-to-br ${bgColors[i % bgColors.length]} flex items-center justify-center text-white font-bold text-lg`}>
                    {initials(member.name)}
                  </div>
                )}
                <div>
                  <h3 className="text-lg font-semibold text-white">{member.name}</h3>
                  <p className="text-sm text-accent-300">{member.designation}</p>
                </div>
              </div>
              <div className="relative">
                <Quote className="absolute -top-1 -left-1 w-4 h-4 text-accent-400/30" />
                <p className="text-sm text-dark-400 leading-relaxed pl-5">{member.about}</p>
              </div>
            </motion.div>
          ))}
        </div>
      </div>
    </div>
    </>
  );
}

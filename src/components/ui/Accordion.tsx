import { createContext, useContext, useState, useCallback, type ReactNode } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { ChevronDown } from 'lucide-react';
import { cn } from '../../utils/helpers';

interface AccordionContextType {
  openIndex: number | null;
  toggle: (index: number) => void;
}

const AccordionContext = createContext<AccordionContextType | null>(null);

function useAccordion() {
  const ctx = useContext(AccordionContext);
  if (!ctx) throw new Error('Accordion.* must be used inside <Accordion>');
  return ctx;
}

interface AccordionProps {
  children: ReactNode;
  className?: string;
}

export default function Accordion({ children, className }: AccordionProps) {
  const [openIndex, setOpenIndex] = useState<number | null>(null);

  const toggle = useCallback((index: number) => {
    setOpenIndex((prev) => (prev === index ? null : index));
  }, []);

  return (
    <AccordionContext.Provider value={{ openIndex, toggle }}>
      <div className={cn('space-y-2', className)}>{children}</div>
    </AccordionContext.Provider>
  );
}

interface ItemProps {
  children: ReactNode;
  index: number;
  className?: string;
}

function Item({ children, index, className }: ItemProps) {
  const { openIndex } = useAccordion();
  const isOpen = openIndex === index;

  return (
    <div
      className={cn(
        'border-b border-white/[0.06]',
        isOpen && 'border-accent-400/20',
        className
      )}
    >
      {children}
    </div>
  );
}

interface HeaderProps {
  children: ReactNode;
  index: number;
  className?: string;
}

function Header({ children, index, className }: HeaderProps) {
  const { openIndex, toggle } = useAccordion();
  const isOpen = openIndex === index;

  return (
    <button
      onClick={() => toggle(index)}
      className={cn(
        'w-full flex items-center justify-between py-4 px-2 text-left hover:bg-white/[0.02] transition-all rounded-lg',
        className
      )}
    >
      <span className="text-white font-medium pr-4 text-sm">{children}</span>
      <ChevronDown
        className={cn(
          'w-4 h-4 text-dark-500 shrink-0 transition-transform duration-300',
          isOpen && 'rotate-180'
        )}
      />
    </button>
  );
}

interface PanelProps {
  children: ReactNode;
  index: number;
}

function Panel({ children, index }: PanelProps) {
  const { openIndex } = useAccordion();
  const isOpen = openIndex === index;

  return (
    <AnimatePresence initial={false}>
      {isOpen && (
        <motion.div
          key="content"
          initial={{ height: 0, opacity: 0 }}
          animate={{ height: 'auto', opacity: 1 }}
          exit={{ height: 0, opacity: 0 }}
          transition={{ duration: 0.3, ease: 'easeInOut' }}
          className="overflow-hidden"
        >
          <p className="px-2 pb-4 text-sm text-dark-400 leading-relaxed">{children}</p>
        </motion.div>
      )}
    </AnimatePresence>
  );
}

Accordion.Item = Item;
Accordion.Header = Header;
Accordion.Panel = Panel;

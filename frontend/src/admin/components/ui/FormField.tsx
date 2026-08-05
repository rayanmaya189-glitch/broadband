import type { InputHTMLAttributes, ReactNode, SelectHTMLAttributes, TextareaHTMLAttributes } from 'react';

function baseField(error?: string) {
  return `w-full rounded-xl border bg-dark-950/60 px-3.5 py-2 text-sm text-white placeholder-dark-500 transition-colors focus:outline-none focus:ring-2 ${
    error
      ? 'border-red-500/50 focus:ring-red-500/40'
      : 'border-white/10 focus:border-accent-500/50 focus:ring-accent-500/30'
  }`;
}

interface FieldWrapProps {
  label?: string;
  hint?: string;
  error?: string;
  children: ReactNode;
  required?: boolean;
}

function FieldWrap({ label, hint, error, children, required }: FieldWrapProps) {
  return (
    <label className="block">
      {label && (
        <span className="mb-1.5 block text-xs font-medium text-dark-300">
          {label}
          {required && <span className="ml-0.5 text-accent-400">*</span>}
        </span>
      )}
      {children}
      {error && <span className="mt-1 block text-xs text-red-400">{error}</span>}
      {hint && !error && <span className="mt-1 block text-xs text-dark-500">{hint}</span>}
    </label>
  );
}

interface TextFieldProps extends Omit<InputHTMLAttributes<HTMLInputElement>, 'className'> {
  label?: string;
  hint?: string;
  error?: string;
  required?: boolean;
}

export function TextField({ label, hint, error, required, ...rest }: TextFieldProps) {
  return (
    <FieldWrap label={label} hint={hint} error={error} required={required}>
      <input className={baseField(error)} required={required} {...rest} />
    </FieldWrap>
  );
}

interface TextAreaProps extends Omit<TextareaHTMLAttributes<HTMLTextAreaElement>, 'className'> {
  label?: string;
  hint?: string;
  error?: string;
  required?: boolean;
}

export function TextArea({ label, hint, error, required, ...rest }: TextAreaProps) {
  return (
    <FieldWrap label={label} hint={hint} error={error} required={required}>
      <textarea className={`${baseField(error)} min-h-[90px] resize-y`} required={required} {...rest} />
    </FieldWrap>
  );
}

interface SelectFieldProps extends Omit<SelectHTMLAttributes<HTMLSelectElement>, 'className'> {
  label?: string;
  hint?: string;
  error?: string;
  required?: boolean;
  options: { value: string; label: string }[];
  placeholder?: string;
  className?: string;
}

export function SelectField({ label, hint, error, required, options, placeholder, className, ...rest }: SelectFieldProps) {
  return (
    <FieldWrap label={label} hint={hint} error={error} required={required}>
      <select className={`${baseField(error)} appearance-none ${className ?? ''}`} required={required} {...rest}>
        {placeholder && <option value="">{placeholder}</option>}
        {options.map((opt) => (
          <option key={opt.value} value={opt.value} className="bg-dark-900 text-white">
            {opt.label}
          </option>
        ))}
      </select>
    </FieldWrap>
  );
}

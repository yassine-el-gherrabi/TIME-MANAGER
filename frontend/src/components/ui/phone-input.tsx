/**
 * Phone Input Component
 *
 * Input field with country code selector for phone numbers.
 * Uses emoji flags for a clean, dependency-free implementation.
 */

import { useState, useEffect, type FC, type ChangeEvent, useMemo } from 'react';
import { ChevronDown } from 'lucide-react';
import { Input } from './input';
import { Button } from './button';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from './dropdown-menu';
import { cn } from '../../lib/utils';

// Common country codes with emoji flags
const COUNTRIES = [
  { code: 'FR', dialCode: '+33', flag: '🇫🇷', name: 'France' },
  { code: 'US', dialCode: '+1', flag: '🇺🇸', name: 'United States' },
  { code: 'GB', dialCode: '+44', flag: '🇬🇧', name: 'United Kingdom' },
  { code: 'DE', dialCode: '+49', flag: '🇩🇪', name: 'Germany' },
  { code: 'ES', dialCode: '+34', flag: '🇪🇸', name: 'Spain' },
  { code: 'IT', dialCode: '+39', flag: '🇮🇹', name: 'Italy' },
  { code: 'PT', dialCode: '+351', flag: '🇵🇹', name: 'Portugal' },
  { code: 'BE', dialCode: '+32', flag: '🇧🇪', name: 'Belgium' },
  { code: 'NL', dialCode: '+31', flag: '🇳🇱', name: 'Netherlands' },
  { code: 'CH', dialCode: '+41', flag: '🇨🇭', name: 'Switzerland' },
  { code: 'AT', dialCode: '+43', flag: '🇦🇹', name: 'Austria' },
  { code: 'PL', dialCode: '+48', flag: '🇵🇱', name: 'Poland' },
  { code: 'SE', dialCode: '+46', flag: '🇸🇪', name: 'Sweden' },
  { code: 'NO', dialCode: '+47', flag: '🇳🇴', name: 'Norway' },
  { code: 'DK', dialCode: '+45', flag: '🇩🇰', name: 'Denmark' },
  { code: 'FI', dialCode: '+358', flag: '🇫🇮', name: 'Finland' },
  { code: 'IE', dialCode: '+353', flag: '🇮🇪', name: 'Ireland' },
  { code: 'CA', dialCode: '+1', flag: '🇨🇦', name: 'Canada' },
  { code: 'AU', dialCode: '+61', flag: '🇦🇺', name: 'Australia' },
  { code: 'JP', dialCode: '+81', flag: '🇯🇵', name: 'Japan' },
  { code: 'CN', dialCode: '+86', flag: '🇨🇳', name: 'China' },
  { code: 'IN', dialCode: '+91', flag: '🇮🇳', name: 'India' },
  { code: 'BR', dialCode: '+55', flag: '🇧🇷', name: 'Brazil' },
  { code: 'MX', dialCode: '+52', flag: '🇲🇽', name: 'Mexico' },
  { code: 'RU', dialCode: '+7', flag: '🇷🇺', name: 'Russia' },
  { code: 'ZA', dialCode: '+27', flag: '🇿🇦', name: 'South Africa' },
  { code: 'AE', dialCode: '+971', flag: '🇦🇪', name: 'UAE' },
  { code: 'SG', dialCode: '+65', flag: '🇸🇬', name: 'Singapore' },
  { code: 'HK', dialCode: '+852', flag: '🇭🇰', name: 'Hong Kong' },
  { code: 'KR', dialCode: '+82', flag: '🇰🇷', name: 'South Korea' },
] as const;

type Country = typeof COUNTRIES[number];

interface PhoneInputProps {
  value: string;
  onChange: (value: string) => void;
  placeholder?: string;
  disabled?: boolean;
  className?: string;
  error?: boolean;
}

/**
 * Parse phone number to extract country code and local number
 */
function parsePhoneNumber(value: string): { country: Country | null; localNumber: string } {
  if (!value) {
    return { country: COUNTRIES[0], localNumber: '' }; // Default to France
  }

  // Find matching country by dial code (longest match first)
  const sortedCountries = [...COUNTRIES].sort((a, b) => b.dialCode.length - a.dialCode.length);

  for (const country of sortedCountries) {
    if (value.startsWith(country.dialCode)) {
      const localNumber = value.slice(country.dialCode.length).trim();
      return { country, localNumber };
    }
  }

  // No matching country code found, default to France
  return { country: COUNTRIES[0], localNumber: value.replace(/^\+/, '') };
}

/**
 * Format local number with spaces for readability
 */
function formatLocalNumber(number: string): string {
  // Remove all non-digit characters
  const digits = number.replace(/\D/g, '');

  // Format with spaces every 2 digits (French style)
  return digits.replace(/(\d{2})(?=\d)/g, '$1 ').trim();
}

export const PhoneInput: FC<PhoneInputProps> = ({
  value,
  onChange,
  placeholder = '6 12 34 56 78',
  disabled = false,
  className,
  error = false,
}) => {
  const parsed = useMemo(() => parsePhoneNumber(value), [value]);
  const [selectedCountry, setSelectedCountry] = useState<Country>(parsed.country ?? COUNTRIES[0]);
  const [localNumber, setLocalNumber] = useState(parsed.localNumber);

  // Update local state when value prop changes
  useEffect(() => {
    const newParsed = parsePhoneNumber(value);
    if (newParsed.country) {
      setSelectedCountry(newParsed.country);
    }
    setLocalNumber(newParsed.localNumber);
  }, [value]);

  const handleCountryChange = (country: Country) => {
    setSelectedCountry(country);
    // Update the full value with new country code
    if (localNumber) {
      onChange(`${country.dialCode}${localNumber.replace(/\s/g, '')}`);
    }
  };

  const handleLocalNumberChange = (e: ChangeEvent<HTMLInputElement>) => {
    const inputValue = e.target.value;
    // Remove all non-digit characters for storage
    const digitsOnly = inputValue.replace(/\D/g, '');

    // Limit to 15 digits (E.164 max)
    const limitedDigits = digitsOnly.slice(0, 15);

    setLocalNumber(formatLocalNumber(limitedDigits));

    // Build full phone number
    if (limitedDigits) {
      onChange(`${selectedCountry.dialCode}${limitedDigits}`);
    } else {
      onChange('');
    }
  };

  return (
    <div className={cn('flex gap-0', className)}>
      <DropdownMenu>
        <DropdownMenuTrigger asChild disabled={disabled}>
          <Button
            variant="outline"
            className={cn(
              'flex items-center gap-1 rounded-r-none border-r-0 px-3 min-w-[100px]',
              error && 'border-destructive',
              disabled && 'cursor-not-allowed opacity-50'
            )}
            type="button"
          >
            <span className="text-lg" role="img" aria-label={selectedCountry.name}>
              {selectedCountry.flag}
            </span>
            <span className="text-sm text-muted-foreground">{selectedCountry.dialCode}</span>
            <ChevronDown className="h-3 w-3 text-muted-foreground" />
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent align="start" className="max-h-[300px] overflow-y-auto w-[240px]">
          {COUNTRIES.map((country) => (
            <DropdownMenuItem
              key={country.code}
              onClick={() => handleCountryChange(country)}
              className="flex items-center gap-2 cursor-pointer"
            >
              <span className="text-lg" role="img" aria-label={country.name}>
                {country.flag}
              </span>
              <span className="flex-1">{country.name}</span>
              <span className="text-sm text-muted-foreground">{country.dialCode}</span>
            </DropdownMenuItem>
          ))}
        </DropdownMenuContent>
      </DropdownMenu>
      <div className="relative flex-1">
        <Input
          type="tel"
          value={localNumber}
          onChange={handleLocalNumberChange}
          placeholder={placeholder}
          disabled={disabled}
          className={cn(
            'rounded-l-none',
            error && 'border-destructive'
          )}
        />
      </div>
    </div>
  );
};

export { COUNTRIES, type Country };

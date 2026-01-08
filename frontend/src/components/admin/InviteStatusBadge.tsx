import * as React from 'react';
import { cva, type VariantProps } from 'class-variance-authority';
import { cn } from '@/lib/utils';

const badgeVariants = cva(
  'inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-semibold transition-colors',
  {
    variants: {
      status: {
        pending: 'bg-yellow-100 text-yellow-800 border border-yellow-200',
        accepted: 'bg-green-100 text-green-800 border border-green-200',
      },
    },
    defaultVariants: {
      status: 'pending',
    },
  }
);

export interface InviteStatusBadgeProps
  extends React.HTMLAttributes<HTMLSpanElement>,
    VariantProps<typeof badgeVariants> {
  hasPassword: boolean;
}

export const InviteStatusBadge: React.FC<InviteStatusBadgeProps> = ({
  className,
  hasPassword,
  ...props
}) => {
  const status = hasPassword ? 'accepted' : 'pending';
  const label = hasPassword ? 'Active' : 'Pending Invite';

  return (
    <span className={cn(badgeVariants({ status }), className)} {...props}>
      {label}
    </span>
  );
};

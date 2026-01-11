/**
 * WelcomeModal Component
 *
 * Displays a welcome modal for new users with role-specific tips.
 * Shows after first login and can be dismissed with "don't show again" option.
 */

import React, { useState } from 'react';
import { Sparkles, Clock, BarChart3, FileText, CheckCircle, Calendar, Users, Settings, Building2, ClipboardList } from 'lucide-react';
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
  DialogFooter,
} from '../ui/dialog';
import { Button } from '../ui/button';
import { Checkbox } from '../ui/checkbox';
import { Label } from '../ui/label';
import { Badge } from '../ui/badge';
import { UserRole } from '../../types/auth';

interface WelcomeModalProps {
  /** Whether the modal is open */
  open: boolean;
  /** User's first name for personalized greeting */
  firstName: string;
  /** User's role to determine which tips to show */
  role: UserRole;
  /** Callback when modal is dismissed */
  onDismiss: (dontShowAgain: boolean) => void;
}

interface Tip {
  icon: React.ReactNode;
  title: string;
  description: string;
  iconBg: string;
  iconColor: string;
}

/**
 * Get tips based on user role (inclusive hierarchy)
 * Each role sees their own tips plus tips from roles below them
 */
const getTipsForRole = (role: UserRole): Tip[] => {
  const employeeTips: Tip[] = [
    {
      icon: <Clock className="h-4 w-4" />,
      title: 'Time Tracking',
      description: 'Clock in/out from the Dashboard or Time Clock page',
      iconBg: 'bg-blue-500/10',
      iconColor: 'text-blue-500',
    },
    {
      icon: <BarChart3 className="h-4 w-4" />,
      title: 'Your KPIs',
      description: 'Track your hours and punctuality with KPI cards',
      iconBg: 'bg-emerald-500/10',
      iconColor: 'text-emerald-500',
    },
    {
      icon: <FileText className="h-4 w-4" />,
      title: 'Absences',
      description: 'Request time off and check your leave balance',
      iconBg: 'bg-violet-500/10',
      iconColor: 'text-violet-500',
    },
  ];

  const managerTips: Tip[] = [
    {
      icon: <CheckCircle className="h-4 w-4" />,
      title: 'Approvals',
      description: 'Review and approve clock entries and absence requests',
      iconBg: 'bg-amber-500/10',
      iconColor: 'text-amber-500',
    },
    {
      icon: <Calendar className="h-4 w-4" />,
      title: 'Team Calendar',
      description: 'View team availability and planned absences',
      iconBg: 'bg-pink-500/10',
      iconColor: 'text-pink-500',
    },
  ];

  const adminTips: Tip[] = [
    {
      icon: <Users className="h-4 w-4" />,
      title: 'User Management',
      description: 'Manage users, teams, and work schedules',
      iconBg: 'bg-cyan-500/10',
      iconColor: 'text-cyan-500',
    },
    {
      icon: <Settings className="h-4 w-4" />,
      title: 'Configuration',
      description: 'Configure absence types and closed days',
      iconBg: 'bg-slate-500/10',
      iconColor: 'text-slate-500',
    },
  ];

  const superAdminTips: Tip[] = [
    {
      icon: <Building2 className="h-4 w-4" />,
      title: 'Organizations',
      description: 'Manage multiple organizations from one place',
      iconBg: 'bg-orange-500/10',
      iconColor: 'text-orange-500',
    },
    {
      icon: <ClipboardList className="h-4 w-4" />,
      title: 'Audit Logs',
      description: 'View system-wide activity for compliance',
      iconBg: 'bg-rose-500/10',
      iconColor: 'text-rose-500',
    },
  ];

  switch (role) {
    case UserRole.SuperAdmin:
      return [...superAdminTips, ...adminTips, ...managerTips, ...employeeTips];
    case UserRole.Admin:
      return [...adminTips, ...managerTips, ...employeeTips];
    case UserRole.Manager:
      return [...managerTips, ...employeeTips];
    case UserRole.Employee:
    default:
      return employeeTips;
  }
};

/**
 * Get role display label and badge class (matches UsersTable colors)
 */
const getRoleConfig = (role: UserRole): { label: string; badgeClass: string } => {
  switch (role) {
    case UserRole.SuperAdmin:
      return { label: 'Super Admin', badgeClass: 'bg-amber-100 text-amber-800 border-amber-200' };
    case UserRole.Admin:
      return { label: 'Admin', badgeClass: 'bg-purple-100 text-purple-800 border-purple-200' };
    case UserRole.Manager:
      return { label: 'Manager', badgeClass: 'bg-blue-100 text-blue-800 border-blue-200' };
    case UserRole.Employee:
    default:
      return { label: 'Employee', badgeClass: 'bg-gray-100 text-gray-800 border-gray-200' };
  }
};

export const WelcomeModal: React.FC<WelcomeModalProps> = ({
  open,
  firstName,
  role,
  onDismiss,
}) => {
  const [dontShowAgain, setDontShowAgain] = useState(true);
  const tips = getTipsForRole(role);
  const roleConfig = getRoleConfig(role);

  const handleDismiss = () => {
    onDismiss(dontShowAgain);
  };

  return (
    <Dialog open={open} onOpenChange={(isOpen) => !isOpen && handleDismiss()}>
      <DialogContent className="sm:max-w-lg max-h-[90vh] flex flex-col">
        {/* Header with icon */}
        <div className="flex flex-col items-center text-center pt-2 flex-shrink-0">
          <div className="mb-4 rounded-full bg-primary/10 p-3">
            <Sparkles className="h-8 w-8 text-primary" />
          </div>
          <DialogHeader className="space-y-2">
            <DialogTitle className="text-2xl font-bold">
              Welcome, {firstName}!
            </DialogTitle>
            <DialogDescription className="flex flex-col items-center gap-2">
              <span>You're signed in as</span>
              <Badge variant="outline" className={`text-xs ${roleConfig.badgeClass}`}>
                {roleConfig.label}
              </Badge>
            </DialogDescription>
          </DialogHeader>
        </div>

        {/* Tips Grid - Scrollable */}
        <div className="py-4 flex-1 overflow-hidden flex flex-col min-h-0">
          <p className="text-sm text-muted-foreground text-center mb-4 flex-shrink-0">
            Here's what you can do:
          </p>
          <div className="overflow-y-auto flex-1 [&::-webkit-scrollbar]:w-2 [&::-webkit-scrollbar-track]:bg-transparent [&::-webkit-scrollbar-thumb]:bg-border [&::-webkit-scrollbar-thumb]:rounded-full">
            <div className="grid gap-3">
              {tips.map((tip, index) => (
                <div
                  key={index}
                  className="flex items-start gap-3 rounded-lg border bg-card p-3 transition-colors hover:bg-accent/50"
                >
                  <div className={`rounded-md p-2 ${tip.iconBg} ${tip.iconColor}`}>
                    {tip.icon}
                  </div>
                  <div className="flex-1 space-y-0.5">
                    <p className="text-sm font-medium leading-none">{tip.title}</p>
                    <p className="text-xs text-muted-foreground">{tip.description}</p>
                  </div>
                </div>
              ))}
            </div>
          </div>
        </div>

        {/* Footer */}
        <DialogFooter className="flex-col gap-4 sm:flex-col border-t pt-4 flex-shrink-0">
          <Button onClick={handleDismiss} className="w-full" size="lg">
            Get Started
          </Button>
          <div className="flex items-center justify-center space-x-2">
            <Checkbox
              id="dont-show-again"
              checked={dontShowAgain}
              onCheckedChange={(checked) => setDontShowAgain(checked === true)}
            />
            <Label
              htmlFor="dont-show-again"
              className="text-xs text-muted-foreground cursor-pointer"
            >
              Don't show this again
            </Label>
          </div>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
};

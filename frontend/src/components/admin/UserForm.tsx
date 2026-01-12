import { useState, useEffect } from 'react';
import type { FC, FormEvent } from 'react';
import { useTranslation } from 'react-i18next';
import { Building2, Users } from 'lucide-react';
import { Button } from '../ui/button';
import { Input } from '../ui/input';
import { Label } from '../ui/label';
import { Badge } from '../ui/badge';
import { Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter } from '../ui/card';
import { UserRole } from '../../types/auth';
import type { UserResponse, CreateUserRequest } from '../../types/user';
import type { OrganizationResponse } from '../../types/organization';
import type { TeamResponse } from '../../types/team';
import { useCurrentUser } from '../../hooks/useAuth';
import { organizationsApi } from '../../api/organizations';
import { teamsApi } from '../../api/teams';

export interface ScheduleOption {
  id: string;
  name: string;
}

/** Result from onSubmit containing the created user's ID for follow-up operations */
export interface SubmitResult {
  userId: string;
}

export interface UserFormProps {
  user?: UserResponse | null;
  /** Submit handler that can optionally return the created user's ID for team/schedule assignment */
  onSubmit: (data: CreateUserRequest) => Promise<SubmitResult | void>;
  onCancel: () => void;
  isLoading?: boolean;
  error?: string;
  /** Use 'sheet' variant when rendering inside a Sheet/Drawer */
  variant?: 'card' | 'sheet';
  /** Available schedules for assignment */
  schedules?: ScheduleOption[];
  /** Callback when schedule should be assigned after user save */
  onScheduleAssign?: (userId: string, scheduleId: string | null) => Promise<void>;
  /** Callback when team should be assigned after user create */
  onTeamAssign?: (userId: string, teamId: string) => Promise<void>;
}

interface FormData {
  email: string;
  first_name: string;
  last_name: string;
  role: UserRole;
  /** Organization ID - for SuperAdmin user creation */
  organization_id: string;
  /** Team ID to assign after creation */
  team_id: string;
  /** Schedule ID to assign - empty string means no change, 'none' means remove */
  schedule_id: string;
}

interface FormErrors {
  email?: string;
  first_name?: string;
  last_name?: string;
}

export const UserForm: FC<UserFormProps> = ({
  user,
  onSubmit,
  onCancel,
  isLoading,
  error,
  variant = 'card',
  schedules = [],
  onScheduleAssign,
  onTeamAssign,
}) => {
  const { t } = useTranslation();
  const currentUser = useCurrentUser();
  const isEditing = !!user;
  const isSuperAdmin = currentUser?.role === UserRole.SuperAdmin;

  const [formData, setFormData] = useState<FormData>({
    email: '',
    first_name: '',
    last_name: '',
    role: UserRole.Employee,
    organization_id: '',
    team_id: '',
    schedule_id: '',
  });

  const [errors, setErrors] = useState<FormErrors>({});

  // Organization and team lists for selectors
  const [organizations, setOrganizations] = useState<OrganizationResponse[]>([]);
  const [teams, setTeams] = useState<TeamResponse[]>([]);
  const [loadingOrgs, setLoadingOrgs] = useState(false);
  const [loadingTeams, setLoadingTeams] = useState(false);

  // Fetch organizations for SuperAdmin
  useEffect(() => {
    if (isSuperAdmin && !isEditing) {
      setLoadingOrgs(true);
      organizationsApi.list({ per_page: 100 })
        .then((response) => {
          setOrganizations(response.data);
          // Pre-select current user's org if available
          if (currentUser?.organization_id && !formData.organization_id) {
            setFormData((prev) => ({ ...prev, organization_id: currentUser.organization_id }));
          }
        })
        .catch(() => setOrganizations([]))
        .finally(() => setLoadingOrgs(false));
    }
  }, [isSuperAdmin, isEditing, currentUser?.organization_id]);

  // Fetch teams when organization changes (for create mode)
  useEffect(() => {
    if (!isEditing) {
      const orgId = isSuperAdmin ? formData.organization_id : currentUser?.organization_id;
      if (orgId) {
        setLoadingTeams(true);
        teamsApi.list({ per_page: 100, organization_id: orgId })
          .then((response) => setTeams(response.teams))
          .catch(() => setTeams([]))
          .finally(() => setLoadingTeams(false));
      } else {
        setTeams([]);
      }
    }
  }, [isEditing, formData.organization_id, isSuperAdmin, currentUser?.organization_id]);

  useEffect(() => {
    if (user) {
      setFormData({
        email: user.email,
        first_name: user.first_name,
        last_name: user.last_name,
        role: user.role,
        organization_id: user.organization_id,
        team_id: '',
        schedule_id: '',
      });
    } else {
      // Reset form when user is cleared (e.g., sheet closed)
      setFormData({
        email: '',
        first_name: '',
        last_name: '',
        role: UserRole.Employee,
        organization_id: currentUser?.organization_id || '',
        team_id: '',
        schedule_id: '',
      });
    }
  }, [user, currentUser?.organization_id]);

  const validateForm = (): boolean => {
    const newErrors: FormErrors = {};

    if (!formData.email.trim()) {
      newErrors.email = t('validation.emailRequired');
    } else if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(formData.email)) {
      newErrors.email = t('validation.invalidEmail');
    }

    if (!formData.first_name.trim()) {
      newErrors.first_name = t('validation.firstNameRequired');
    }

    if (!formData.last_name.trim()) {
      newErrors.last_name = t('validation.lastNameRequired');
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault();
    if (!validateForm()) return;

    // Build user data for submission
    const { schedule_id, team_id, organization_id, ...baseUserData } = formData;

    // Include organization_id for SuperAdmin creating new users
    const userData: CreateUserRequest = {
      ...baseUserData,
      ...(isSuperAdmin && !isEditing && organization_id ? { organization_id } : {}),
    };

    const result = await onSubmit(userData);

    // Determine user ID for follow-up operations
    const targetUserId = isEditing ? user?.id : result?.userId;

    if (targetUserId) {
      // Handle team assignment for new users
      if (!isEditing && onTeamAssign && team_id) {
        await onTeamAssign(targetUserId, team_id);
      }

      // Handle schedule assignment (both create and edit modes)
      if (onScheduleAssign && schedule_id) {
        const scheduleToAssign = schedule_id === 'none' ? null : schedule_id;
        await onScheduleAssign(targetUserId, scheduleToAssign);
      }
    }
  };

  const handleChange = (field: keyof FormData, value: string) => {
    setFormData((prev) => ({ ...prev, [field]: value }));
    if (errors[field as keyof FormErrors]) {
      setErrors((prev) => ({ ...prev, [field]: undefined }));
    }
  };

  const formContent = (
    <>
      {error && (
        <div className="p-3 text-sm text-destructive bg-destructive/10 border border-destructive rounded-md">
          {error}
        </div>
      )}

      <div className="space-y-2">
        <Label htmlFor="email">{t('common.email')}</Label>
        <Input
          id="email"
          type="email"
          value={formData.email}
          onChange={(e) => handleChange('email', e.target.value)}
          error={errors.email}
          disabled={isLoading}
          autoComplete="email"
        />
      </div>

      <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
        <div className="space-y-2">
          <Label htmlFor="first_name">{t('users.firstName')}</Label>
          <Input
            id="first_name"
            type="text"
            value={formData.first_name}
            onChange={(e) => handleChange('first_name', e.target.value)}
            error={errors.first_name}
            disabled={isLoading}
            autoComplete="given-name"
          />
        </div>

        <div className="space-y-2">
          <Label htmlFor="last_name">{t('users.lastName')}</Label>
          <Input
            id="last_name"
            type="text"
            value={formData.last_name}
            onChange={(e) => handleChange('last_name', e.target.value)}
            error={errors.last_name}
            disabled={isLoading}
            autoComplete="family-name"
          />
        </div>
      </div>

      <div className="space-y-2">
        <Label htmlFor="role">{t('users.role')}</Label>
        <select
          id="role"
          value={formData.role}
          onChange={(e) => handleChange('role', e.target.value)}
          disabled={isLoading}
          className="flex h-9 w-full rounded-md border border-input bg-background px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50"
        >
          <option value={UserRole.Employee}>{t('roles.employee')}</option>
          <option value={UserRole.Manager}>{t('roles.manager')}</option>
          <option value={UserRole.Admin}>{t('roles.admin')}</option>
          <option value={UserRole.SuperAdmin}>{t('roles.superAdmin')}</option>
        </select>
      </div>

      {/* Organization field - SuperAdmin can select, Admin sees badge */}
      {!isEditing && (
        <div className="space-y-2">
          <Label htmlFor="organization" className="flex items-center gap-2">
            <Building2 className="h-4 w-4" />
            {t('users.organization')}
          </Label>
          {isSuperAdmin ? (
            <select
              id="organization"
              value={formData.organization_id}
              onChange={(e) => handleChange('organization_id', e.target.value)}
              disabled={isLoading || loadingOrgs}
              className="flex h-9 w-full rounded-md border border-input bg-background px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50"
            >
              <option value="">{t('users.selectOrganization')}</option>
              {organizations.map((org) => (
                <option key={org.id} value={org.id}>
                  {org.name}
                </option>
              ))}
            </select>
          ) : (
            <div className="flex items-center gap-2 h-9">
              <Badge variant="secondary" className="text-sm">
                {currentUser?.organization_name || t('users.yourOrganization')}
              </Badge>
            </div>
          )}
        </div>
      )}

      {/* Team assignment - optional, filter by selected organization */}
      {!isEditing && (
        <div className="space-y-2">
          <Label htmlFor="team" className="flex items-center gap-2">
            <Users className="h-4 w-4" />
            {t('users.teamOptional')}
          </Label>
          <select
            id="team"
            value={formData.team_id}
            onChange={(e) => handleChange('team_id', e.target.value)}
            disabled={isLoading || loadingTeams || teams.length === 0}
            className="flex h-9 w-full rounded-md border border-input bg-background px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50"
          >
            <option value="">{t('users.noTeamAssignment')}</option>
            {teams.map((team) => (
              <option key={team.id} value={team.id}>
                {team.name}
              </option>
            ))}
          </select>
          <p className="text-xs text-muted-foreground">
            {loadingTeams ? t('users.loadingTeams') : teams.length === 0 ? t('users.noTeamsAvailable') : t('users.assignToTeam')}
          </p>
        </div>
      )}

      {/* Schedule assignment - show in both create and edit modes */}
      {schedules.length > 0 && (
        <div className="space-y-2">
          <Label htmlFor="schedule">{t('users.personalSchedule')}</Label>
          <select
            id="schedule"
            value={formData.schedule_id}
            onChange={(e) => handleChange('schedule_id', e.target.value)}
            disabled={isLoading}
            className="flex h-9 w-full rounded-md border border-input bg-background px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50"
          >
            <option value="">{isEditing ? t('users.noChange') : t('users.useTeamDefault')}</option>
            {isEditing && <option value="none">{t('users.removePersonalSchedule')}</option>}
            {schedules.map((schedule) => (
              <option key={schedule.id} value={schedule.id}>
                {schedule.name}
              </option>
            ))}
          </select>
          <p className="text-xs text-muted-foreground">
            {isEditing ? t('users.personalScheduleOverride') : t('users.assignPersonalSchedule')}
          </p>
        </div>
      )}
    </>
  );

  const formButtons = (
    <>
      <Button type="button" variant="outline" onClick={onCancel} disabled={isLoading}>
        {t('common.cancel')}
      </Button>
      <Button type="submit" disabled={isLoading}>
        {isLoading ? (isEditing ? t('common.saving') : t('common.creating')) : isEditing ? t('common.save') : t('common.create')}
      </Button>
    </>
  );

  // Sheet variant - no Card wrapper
  if (variant === 'sheet') {
    return (
      <form onSubmit={handleSubmit} className="flex flex-col gap-4 py-4">
        <div className="space-y-4">
          {formContent}
        </div>
        <div className="flex justify-end gap-4 pt-4 border-t">
          {formButtons}
        </div>
      </form>
    );
  }

  // Default Card variant
  return (
    <Card className="w-full max-w-lg">
      <CardHeader>
        <CardTitle>{isEditing ? t('users.editUser') : t('users.addUser')}</CardTitle>
        <CardDescription>
          {isEditing
            ? t('users.editUserDescription')
            : t('users.addUserDescription')}
        </CardDescription>
      </CardHeader>
      <form onSubmit={handleSubmit}>
        <CardContent className="space-y-4">
          {formContent}
        </CardContent>
        <CardFooter className="flex justify-end gap-4">
          {formButtons}
        </CardFooter>
      </form>
    </Card>
  );
};

import React from 'react';
import { Input } from '../ui/input';
import { Label } from '../ui/label';
import { UserRole } from '../../types/auth';

export interface UserFiltersProps {
  search: string;
  role: UserRole | '';
  onSearchChange: (value: string) => void;
  onRoleChange: (value: UserRole | '') => void;
}

export const UserFilters: React.FC<UserFiltersProps> = ({
  search,
  role,
  onSearchChange,
  onRoleChange,
}) => {
  return (
    <div className="flex flex-col sm:flex-row gap-4 mb-6">
      <div className="flex-1">
        <Label htmlFor="search" className="sr-only">
          Search
        </Label>
        <Input
          id="search"
          type="text"
          placeholder="Search by name or email..."
          value={search}
          onChange={(e) => onSearchChange(e.target.value)}
          className="w-full"
        />
      </div>
      <div className="w-full sm:w-48">
        <Label htmlFor="role-filter" className="sr-only">
          Filter by Role
        </Label>
        <select
          id="role-filter"
          value={role}
          onChange={(e) => onRoleChange(e.target.value as UserRole | '')}
          className="flex h-9 w-full rounded-md border border-input bg-background px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring"
        >
          <option value="">All Roles</option>
          <option value={UserRole.Admin}>Admin</option>
          <option value={UserRole.Manager}>Manager</option>
          <option value={UserRole.Employee}>Employee</option>
        </select>
      </div>
    </div>
  );
};

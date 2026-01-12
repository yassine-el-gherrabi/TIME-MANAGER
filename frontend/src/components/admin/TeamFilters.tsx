/**
 * Team Filters Component
 *
 * Provides search and manager filter controls for the teams list.
 */

import React from 'react';
import { Input } from '../ui/input';
import { Label } from '../ui/label';

export interface TeamFiltersProps {
  search: string;
  managerId: string;
  onSearchChange: (value: string) => void;
  onManagerChange: (value: string) => void;
  /** Available managers for the dropdown */
  managers: Array<{ id: string; name: string }>;
}

export const TeamFilters: React.FC<TeamFiltersProps> = ({
  search,
  managerId,
  onSearchChange,
  onManagerChange,
  managers,
}) => {
  return (
    <div className="flex flex-col sm:flex-row gap-4 mb-6">
      <div className="flex-1">
        <Label htmlFor="team-search" className="sr-only">
          Search
        </Label>
        <Input
          id="team-search"
          type="text"
          placeholder="Search by team name..."
          value={search}
          onChange={(e) => onSearchChange(e.target.value)}
          className="w-full"
        />
      </div>
      <div className="w-full sm:w-56">
        <Label htmlFor="manager-filter" className="sr-only">
          Filter by Manager
        </Label>
        <select
          id="manager-filter"
          value={managerId}
          onChange={(e) => onManagerChange(e.target.value)}
          className="flex h-9 w-full rounded-md border border-input bg-background px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring"
        >
          <option value="">All Managers</option>
          {managers.map((manager) => (
            <option key={manager.id} value={manager.id}>
              {manager.name}
            </option>
          ))}
        </select>
      </div>
    </div>
  );
};

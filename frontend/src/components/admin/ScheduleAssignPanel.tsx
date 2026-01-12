/**
 * Schedule Assign Panel Component
 *
 * Sheet panel for assigning a schedule to users.
 * Displays users with checkbox selection for bulk assignment.
 */

import { useState, useEffect, useCallback } from 'react';
import type { FC } from 'react';
import { toast } from 'sonner';
import { Loader2, Search } from 'lucide-react';
import { Button } from '../ui/button';
import { Input } from '../ui/input';
import { Checkbox } from '../ui/checkbox';
import { schedulesApi } from '../../api/schedules';
import { usersApi } from '../../api/users';
import { mapErrorToMessage } from '../../utils/errorHandling';
import type { WorkScheduleWithDays } from '../../types/schedule';
import type { UserResponse } from '../../types/user';

export interface ScheduleAssignPanelProps {
  schedule: WorkScheduleWithDays | null;
  onClose: () => void;
}

export const ScheduleAssignPanel: FC<ScheduleAssignPanelProps> = ({
  schedule,
  onClose,
}) => {
  const [users, setUsers] = useState<UserResponse[]>([]);
  const [loading, setLoading] = useState(true);
  const [searchQuery, setSearchQuery] = useState('');
  const [selectedUserIds, setSelectedUserIds] = useState<Set<string>>(new Set());
  const [assigning, setAssigning] = useState(false);

  // Load users
  const loadUsers = useCallback(async () => {
    setLoading(true);
    try {
      const response = await usersApi.list({ per_page: 100 });
      setUsers(response.data);
      // Start with no users selected - user will choose who to assign
      setSelectedUserIds(new Set());
    } catch (err) {
      toast.error(mapErrorToMessage(err));
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    if (schedule) {
      loadUsers();
    }
  }, [schedule, loadUsers]);

  // Filter users by search query
  const filteredUsers = users.filter((user) => {
    if (!searchQuery.trim()) return true;
    const query = searchQuery.toLowerCase();
    return (
      user.first_name.toLowerCase().includes(query) ||
      user.last_name.toLowerCase().includes(query) ||
      user.email.toLowerCase().includes(query)
    );
  });

  // Toggle user selection
  const handleToggleUser = (userId: string) => {
    setSelectedUserIds((prev) => {
      const newSet = new Set(prev);
      if (newSet.has(userId)) {
        newSet.delete(userId);
      } else {
        newSet.add(userId);
      }
      return newSet;
    });
  };

  // Select all filtered users
  const handleSelectAll = () => {
    setSelectedUserIds((prev) => {
      const newSet = new Set(prev);
      filteredUsers.forEach((user) => newSet.add(user.id));
      return newSet;
    });
  };

  // Clear all selections
  const handleClearAll = () => {
    setSelectedUserIds(new Set());
  };

  // Assign schedule to selected users
  const handleAssign = async () => {
    if (!schedule || selectedUserIds.size === 0) return;

    setAssigning(true);
    try {
      // Assign schedule to all selected users
      const selectedUsers = users.filter((user) => selectedUserIds.has(user.id));

      for (const user of selectedUsers) {
        await schedulesApi.assignToUser(user.id, {
          schedule_id: schedule.id,
        });
      }

      toast.success(`Schedule assigned to ${selectedUsers.length} user${selectedUsers.length !== 1 ? 's' : ''}`);
      onClose();
    } catch (err) {
      toast.error(mapErrorToMessage(err));
    } finally {
      setAssigning(false);
    }
  };

  if (!schedule) {
    return null;
  }

  if (loading) {
    return (
      <div className="flex items-center justify-center py-8">
        <Loader2 className="h-6 w-6 animate-spin text-muted-foreground" />
      </div>
    );
  }

  return (
    <div className="flex flex-col gap-4 py-4">
      {/* Search and bulk actions */}
      <div className="space-y-3">
        <div className="relative">
          <Search className="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground" />
          <Input
            type="text"
            placeholder="Search users..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="pl-9"
          />
        </div>
        <div className="flex items-center justify-between text-sm">
          <span className="text-muted-foreground">
            {selectedUserIds.size} selected
          </span>
          <div className="flex gap-2">
            <Button variant="ghost" size="sm" onClick={handleSelectAll}>
              Select all
            </Button>
            <Button variant="ghost" size="sm" onClick={handleClearAll}>
              Clear
            </Button>
          </div>
        </div>
      </div>

      {/* Users list */}
      <div className="max-h-80 overflow-y-auto space-y-1 border rounded-md p-2">
        {filteredUsers.length === 0 ? (
          <p className="text-sm text-muted-foreground text-center py-4">
            {searchQuery ? 'No matching users found' : 'No users available'}
          </p>
        ) : (
          filteredUsers.map((user) => {
            const isSelected = selectedUserIds.has(user.id);

            return (
              <div
                key={user.id}
                className={`flex items-center gap-3 p-2 rounded-md cursor-pointer hover:bg-muted/50 ${
                  isSelected ? 'bg-muted/30' : ''
                }`}
                onClick={() => handleToggleUser(user.id)}
              >
                <Checkbox
                  checked={isSelected}
                  onCheckedChange={() => handleToggleUser(user.id)}
                />
                <div className="flex-1 min-w-0">
                  <div className="text-sm font-medium truncate">
                    {user.first_name} {user.last_name}
                  </div>
                  <div className="text-xs text-muted-foreground truncate">
                    {user.email}
                  </div>
                </div>
              </div>
            );
          })
        )}
      </div>

      {/* Actions */}
      <div className="flex justify-end gap-2 pt-4 border-t">
        <Button variant="outline" onClick={onClose} disabled={assigning}>
          Cancel
        </Button>
        <Button onClick={handleAssign} disabled={assigning || selectedUserIds.size === 0}>
          {assigning ? (
            <>
              <Loader2 className="h-4 w-4 animate-spin mr-2" />
              Assigning...
            </>
          ) : (
            `Assign to ${selectedUserIds.size} user${selectedUserIds.size !== 1 ? 's' : ''}`
          )}
        </Button>
      </div>
    </div>
  );
};

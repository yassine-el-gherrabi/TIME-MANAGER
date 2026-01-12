/**
 * Team Members Panel Component
 *
 * Sheet panel for managing team members.
 * Allows viewing, adding, and removing members.
 */

import { useState, useEffect, useCallback } from 'react';
import type { FC } from 'react';
import { toast } from 'sonner';
import { Loader2, UserPlus, UserMinus, Search } from 'lucide-react';
import { Button } from '../ui/button';
import { Input } from '../ui/input';
import { teamsApi } from '../../api/teams';
import { usersApi } from '../../api/users';
import { mapErrorToMessage } from '../../utils/errorHandling';
import type { TeamResponse, TeamMemberInfo, TeamWithMembers } from '../../types/team';
import type { UserResponse } from '../../types/user';

export interface TeamMembersPanelProps {
  team: TeamResponse | null;
  onClose: () => void;
  onMembersChanged: () => void;
}

export const TeamMembersPanel: FC<TeamMembersPanelProps> = ({
  team,
  onClose,
  onMembersChanged,
}) => {
  const [members, setMembers] = useState<TeamMemberInfo[]>([]);
  const [availableUsers, setAvailableUsers] = useState<UserResponse[]>([]);
  const [loading, setLoading] = useState(true);
  const [searchQuery, setSearchQuery] = useState('');
  const [showAddSection, setShowAddSection] = useState(false);
  const [addingUserId, setAddingUserId] = useState<string | null>(null);
  const [removingUserId, setRemovingUserId] = useState<string | null>(null);

  // Load team members and available users
  const loadData = useCallback(async () => {
    if (!team) return;

    setLoading(true);
    try {
      // Load team with members
      const teamData = await teamsApi.get(team.id, true) as TeamWithMembers;
      setMembers(teamData.members);

      // Load all users to find available ones
      const usersResponse = await usersApi.list({ per_page: 100 });
      const memberIds = new Set(teamData.members.map((m) => m.user_id));

      // Filter out users who are already members
      const available = usersResponse.data.filter(
        (u) => !memberIds.has(u.id)
      );
      setAvailableUsers(available);
    } catch (err) {
      toast.error(mapErrorToMessage(err));
    } finally {
      setLoading(false);
    }
  }, [team]);

  useEffect(() => {
    loadData();
  }, [loadData]);

  // Handle adding a member
  const handleAddMember = async (userId: string) => {
    if (!team) return;

    setAddingUserId(userId);
    try {
      await teamsApi.addMember(team.id, { user_id: userId });
      toast.success('Member added successfully');
      await loadData();
      onMembersChanged();
    } catch (err) {
      toast.error(mapErrorToMessage(err));
    } finally {
      setAddingUserId(null);
    }
  };

  // Handle removing a member
  const handleRemoveMember = async (userId: string, userName: string) => {
    if (!team) return;

    setRemovingUserId(userId);
    try {
      await teamsApi.removeMember(team.id, userId);
      toast.success(`${userName} has been removed from the team`);
      await loadData();
      onMembersChanged();
    } catch (err) {
      toast.error(mapErrorToMessage(err));
    } finally {
      setRemovingUserId(null);
    }
  };

  // Filter available users by search query
  const filteredAvailableUsers = availableUsers.filter((user) => {
    if (!searchQuery.trim()) return true;
    const query = searchQuery.toLowerCase();
    return (
      user.first_name.toLowerCase().includes(query) ||
      user.last_name.toLowerCase().includes(query) ||
      user.email.toLowerCase().includes(query)
    );
  });

  if (!team) {
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
    <div className="flex flex-col gap-6 py-4">
      {/* Members count */}
      <div className="text-sm text-muted-foreground">
        {members.length} member{members.length !== 1 ? 's' : ''} in this team
      </div>

      {/* Add member section toggle */}
      {!showAddSection && (
        <Button
          variant="outline"
          className="w-full justify-start gap-2"
          onClick={() => setShowAddSection(true)}
        >
          <UserPlus className="h-4 w-4" />
          Add a member
        </Button>
      )}

      {/* Add member section */}
      {showAddSection && (
        <div className="border rounded-lg p-4 space-y-4">
          <div className="flex items-center justify-between">
            <h4 className="text-sm font-medium">Add Member</h4>
            <Button
              variant="ghost"
              size="sm"
              onClick={() => {
                setShowAddSection(false);
                setSearchQuery('');
              }}
            >
              Cancel
            </Button>
          </div>

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

          <div className="max-h-48 overflow-y-auto space-y-2">
            {filteredAvailableUsers.length === 0 ? (
              <p className="text-sm text-muted-foreground text-center py-2">
                {searchQuery ? 'No matching users found' : 'No users available to add'}
              </p>
            ) : (
              filteredAvailableUsers.map((user) => (
                <div
                  key={user.id}
                  className="flex items-center justify-between p-2 rounded-md hover:bg-muted/50"
                >
                  <div>
                    <div className="text-sm font-medium">
                      {user.first_name} {user.last_name}
                    </div>
                    <div className="text-xs text-muted-foreground">
                      {user.email}
                    </div>
                  </div>
                  <Button
                    size="sm"
                    onClick={() => handleAddMember(user.id)}
                    disabled={addingUserId === user.id}
                  >
                    {addingUserId === user.id ? (
                      <Loader2 className="h-4 w-4 animate-spin" />
                    ) : (
                      'Add'
                    )}
                  </Button>
                </div>
              ))
            )}
          </div>
        </div>
      )}

      {/* Current members list */}
      <div className="space-y-2">
        <h4 className="text-sm font-medium">Current Members</h4>
        {members.length === 0 ? (
          <p className="text-sm text-muted-foreground text-center py-4">
            No members in this team yet
          </p>
        ) : (
          <div className="space-y-2">
            {members.map((member) => (
              <div
                key={member.user_id}
                className="flex items-center justify-between p-3 border rounded-md"
              >
                <div>
                  <div className="font-medium">
                    {member.first_name} {member.last_name}
                  </div>
                  <div className="text-sm text-muted-foreground">
                    {member.email}
                  </div>
                </div>
                <Button
                  variant="ghost"
                  size="sm"
                  className="text-destructive hover:text-destructive hover:bg-destructive/10"
                  onClick={() =>
                    handleRemoveMember(
                      member.user_id,
                      `${member.first_name} ${member.last_name}`
                    )
                  }
                  disabled={removingUserId === member.user_id}
                >
                  {removingUserId === member.user_id ? (
                    <Loader2 className="h-4 w-4 animate-spin" />
                  ) : (
                    <UserMinus className="h-4 w-4" />
                  )}
                </Button>
              </div>
            ))}
          </div>
        )}
      </div>

      {/* Close button */}
      <div className="flex justify-end pt-4 border-t">
        <Button variant="outline" onClick={onClose}>
          Done
        </Button>
      </div>
    </div>
  );
};

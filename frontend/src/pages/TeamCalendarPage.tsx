/**
 * Team Calendar Page (Manager)
 *
 * Visual calendar showing team absences with monthly navigation.
 */

import { useState, useEffect, useMemo } from 'react';
import {
  format,
  startOfMonth,
  endOfMonth,
  eachDayOfInterval,
  isToday,
  isWeekend,
  addMonths,
  subMonths,
} from 'date-fns';
import { ChevronLeft, ChevronRight, Loader2, Calendar } from 'lucide-react';
import { logger } from '../utils/logger';
import { Card, CardContent, CardHeader, CardTitle } from '../components/ui/card';
import { Button } from '../components/ui/button';
import { cn } from '../lib/utils';
import { absencesApi } from '../api/absences';
import { absenceTypesApi } from '../api/absenceTypes';
import { OrgTeamFilter, useOrgTeamFilter } from '../components/filters';
import type { Absence, AbsenceType } from '../types/absence';
import { AbsenceStatus } from '../types/absence';

interface UserAbsence {
  userId: string;
  userName: string;
  absences: Absence[];
}

export function TeamCalendarPage() {
  // Org/Team filter state
  const {
    selectedOrgId,
    selectedTeamId,
    setSelectedOrgId,
    setSelectedTeamId,
  } = useOrgTeamFilter();

  const [currentMonth, setCurrentMonth] = useState(new Date());
  const [loading, setLoading] = useState(true);
  const [userAbsences, setUserAbsences] = useState<UserAbsence[]>([]);
  const [absenceTypes, setAbsenceTypes] = useState<AbsenceType[]>([]);

  // Calculate month range
  const monthStart = startOfMonth(currentMonth);
  const monthEnd = endOfMonth(currentMonth);
  const days = eachDayOfInterval({ start: monthStart, end: monthEnd });

  // Create type lookup map
  const typeMap = useMemo(() => {
    const map: Record<string, AbsenceType> = {};
    absenceTypes.forEach((t) => {
      map[t.id] = t;
    });
    return map;
  }, [absenceTypes]);

  // Load absences for current month
  useEffect(() => {
    const loadData = async () => {
      setLoading(true);
      try {
        // Build absence list params with filters
        const absenceParams: {
          start_date: string;
          end_date: string;
          status: AbsenceStatus;
          per_page: number;
          organization_id?: string;
          team_id?: string;
        } = {
          start_date: format(monthStart, 'yyyy-MM-dd'),
          end_date: format(monthEnd, 'yyyy-MM-dd'),
          status: AbsenceStatus.Approved,
          per_page: 200,
        };
        if (selectedOrgId) absenceParams.organization_id = selectedOrgId;
        if (selectedTeamId) absenceParams.team_id = selectedTeamId;

        const [types, response] = await Promise.all([
          absenceTypesApi.list(selectedOrgId ? { organization_id: selectedOrgId } : {}),
          absencesApi.list(absenceParams),
        ]);

        setAbsenceTypes(types);

        // Group absences by user
        const userMap = new Map<string, UserAbsence>();
        response.data.forEach((absence) => {
          const userId = absence.user_id;
          if (!userMap.has(userId)) {
            userMap.set(userId, {
              userId,
              userName: absence.user_name || 'Unknown User',
              absences: [],
            });
          }
          userMap.get(userId)!.absences.push(absence);
        });

        setUserAbsences(Array.from(userMap.values()));
      } catch (err) {
        logger.error('Failed to load calendar data', err, { component: 'TeamCalendarPage', action: 'loadData' });
      } finally {
        setLoading(false);
      }
    };

    loadData();
  }, [currentMonth, selectedOrgId, selectedTeamId]);

  // Check if a user has absence on a specific day
  const getAbsenceForDay = (userId: string, day: Date): Absence | null => {
    const dayStr = format(day, 'yyyy-MM-dd');
    const user = userAbsences.find((u) => u.userId === userId);
    if (!user) return null;

    return user.absences.find((absence) => {
      return dayStr >= absence.start_date && dayStr <= absence.end_date;
    }) || null;
  };

  // Navigation handlers
  const goToPreviousMonth = () => setCurrentMonth(subMonths(currentMonth, 1));
  const goToNextMonth = () => setCurrentMonth(addMonths(currentMonth, 1));
  const goToToday = () => setCurrentMonth(new Date());

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4">
        <div>
          <h1 className="text-2xl font-bold tracking-tight">Team Calendar</h1>
          <p className="text-muted-foreground">
            View team absences across the month
          </p>
        </div>

        {/* Month Navigation */}
        <div className="flex items-center gap-2">
          <Button variant="outline" size="icon" onClick={goToPreviousMonth}>
            <ChevronLeft className="h-4 w-4" />
          </Button>
          <Button variant="outline" onClick={goToToday}>
            Today
          </Button>
          <h2 className="text-lg font-semibold min-w-[120px] sm:min-w-[150px] text-center">
            {format(currentMonth, 'MMMM yyyy')}
          </h2>
          <Button variant="outline" size="icon" onClick={goToNextMonth}>
            <ChevronRight className="h-4 w-4" />
          </Button>
        </div>
      </div>

      {/* Org/Team Filters */}
      <Card>
        <CardContent className="py-3">
          <OrgTeamFilter
            showTeamFilter={true}
            selectedOrgId={selectedOrgId}
            selectedTeamId={selectedTeamId}
            onOrgChange={setSelectedOrgId}
            onTeamChange={setSelectedTeamId}
          />
        </CardContent>
      </Card>

      {/* Legend */}
      <Card>
        <CardContent className="py-3">
          <div className="flex flex-wrap items-center gap-4">
            <span className="text-sm text-muted-foreground">Legend:</span>
            {absenceTypes.map((type) => (
              <div key={type.id} className="flex items-center gap-1.5">
                <div
                  className="h-3 w-3 rounded"
                  style={{ backgroundColor: type.color }}
                />
                <span className="text-sm">{type.name}</span>
              </div>
            ))}
          </div>
        </CardContent>
      </Card>

      {/* Calendar Grid */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2 text-base">
            <Calendar className="h-5 w-5" />
            Team Absences
          </CardTitle>
        </CardHeader>
        <CardContent>
          {loading ? (
            <div className="flex items-center justify-center py-8">
              <Loader2 className="h-8 w-8 animate-spin text-muted-foreground" />
            </div>
          ) : (
            <div className="overflow-x-auto">
              <table className="w-full border-collapse text-sm">
                <thead>
                  <tr>
                    <th className="sticky left-0 z-10 bg-background border-b px-2 sm:px-3 py-2 text-left font-medium min-w-[100px] sm:min-w-[150px]">
                      Team Member
                    </th>
                    {days.map((day) => (
                      <th
                        key={day.toISOString()}
                        className={cn(
                          'border-b px-1 py-2 text-center font-normal min-w-[30px]',
                          isWeekend(day) && 'bg-muted/50',
                          isToday(day) && 'bg-primary/10'
                        )}
                      >
                        <div className="text-xs text-muted-foreground">
                          {format(day, 'EEE')}
                        </div>
                        <div className={cn(
                          'text-sm',
                          isToday(day) && 'font-bold text-primary'
                        )}>
                          {format(day, 'd')}
                        </div>
                      </th>
                    ))}
                  </tr>
                </thead>
                <tbody>
                  {userAbsences.length === 0 ? (
                    <tr>
                      <td
                        colSpan={days.length + 1}
                        className="text-center py-8 text-muted-foreground"
                      >
                        No approved absences for this month
                      </td>
                    </tr>
                  ) : (
                    userAbsences.map((user) => (
                      <tr key={user.userId} className="hover:bg-muted/50">
                        <td className="sticky left-0 z-10 bg-background border-b px-2 sm:px-3 py-2 font-medium truncate max-w-[100px] sm:max-w-none">
                          {user.userName}
                        </td>
                        {days.map((day) => {
                          const absence = getAbsenceForDay(user.userId, day);
                          const type = absence ? typeMap[absence.type_id] : null;

                          return (
                            <td
                              key={day.toISOString()}
                              className={cn(
                                'border-b px-1 py-2 text-center',
                                isWeekend(day) && 'bg-muted/50'
                              )}
                            >
                              {absence && type && (
                                <div
                                  className="h-6 w-full rounded"
                                  style={{ backgroundColor: type.color }}
                                  title={`${type.name}: ${absence.start_date} - ${absence.end_date}`}
                                />
                              )}
                            </td>
                          );
                        })}
                      </tr>
                    ))
                  )}
                </tbody>
              </table>
            </div>
          )}
        </CardContent>
      </Card>
    </div>
  );
}

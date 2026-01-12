/**
 * Window Management Drawer Component
 *
 * Handles break window management for policies.
 */

import { Plus, Trash2, Coffee } from 'lucide-react';
import { Button } from '../../../components/ui/button';
import {
  Sheet,
  SheetContent,
  SheetHeader,
  SheetTitle,
  SheetDescription,
} from '../../../components/ui/sheet';
import { Input } from '../../../components/ui/input';
import { Label } from '../../../components/ui/label';
import { Switch } from '../../../components/ui/switch';
import { Badge } from '../../../components/ui/badge';
import { Checkbox } from '../../../components/ui/checkbox';
import { DAYS_OF_WEEK, getDayLabel, formatBreakDuration } from '../../../types/break';
import type { BreakWindowResponse } from '../../../types/break';
import type { WindowDrawerState, WindowFormData } from './types';

interface WindowManagementDrawerProps {
  windowDrawer: WindowDrawerState;
  windowFormData: WindowFormData;
  onWindowDrawerChange: (state: Partial<WindowDrawerState>) => void;
  onWindowFormDataChange: (data: WindowFormData) => void;
  onAddWindow: () => void;
  onDeleteWindow: (windowId: string) => void;
}

export function WindowManagementDrawer({
  windowDrawer,
  windowFormData,
  onWindowDrawerChange,
  onWindowFormDataChange,
  onAddWindow,
  onDeleteWindow,
}: WindowManagementDrawerProps) {
  const formatTime = (time: string) => time.substring(0, 5);

  return (
    <Sheet
      open={windowDrawer.open}
      onOpenChange={(open) => onWindowDrawerChange({ open })}
    >
      <SheetContent className="overflow-y-auto sm:max-w-xl">
        <SheetHeader>
          <SheetTitle>Manage Break Windows</SheetTitle>
          <SheetDescription>
            Configure when breaks must or can be taken for "{windowDrawer.policy?.name}".
            <br />
            <span className="text-xs">
              {windowDrawer.policy?.tracking_mode === 'explicit_tracking'
                ? 'Employees will see break buttons during these windows.'
                : 'Break time will be auto-deducted based on these windows.'}
            </span>
          </SheetDescription>
        </SheetHeader>

        <div className="space-y-6 py-4">
          {/* Empty State Guidance */}
          {windowDrawer.policy && windowDrawer.policy.windows.length === 0 && (
            <div className="text-center py-6 border border-dashed rounded-md">
              <Coffee className="h-8 w-8 mx-auto text-muted-foreground mb-2" />
              <p className="text-sm font-medium">No break windows configured</p>
              <p className="text-xs text-muted-foreground mt-1 max-w-[280px] mx-auto">
                Add at least one break window to define when breaks should be taken. You can select multiple days at once.
              </p>
            </div>
          )}

          {/* Existing Windows */}
          {windowDrawer.policy && windowDrawer.policy.windows.length > 0 && (
            <div className="space-y-2">
              <Label>Current Windows ({windowDrawer.policy.windows.length})</Label>
              <div className="rounded-md border divide-y max-h-[200px] overflow-y-auto">
                {windowDrawer.policy.windows.map((window: BreakWindowResponse) => (
                  <div key={window.id} className="flex items-center justify-between p-3">
                    <div className="space-y-1">
                      <div className="flex items-center gap-2">
                        <Badge variant="outline">{getDayLabel(window.day_of_week)}</Badge>
                        <span className="text-sm">
                          {formatTime(window.window_start)} - {formatTime(window.window_end)}
                        </span>
                      </div>
                      <div className="flex items-center gap-2 text-xs text-muted-foreground">
                        <span>
                          {formatBreakDuration(window.min_duration_minutes)} - {formatBreakDuration(window.max_duration_minutes)}
                        </span>
                        {window.is_mandatory && (
                          <Badge variant="secondary" className="text-xs">
                            Mandatory
                          </Badge>
                        )}
                      </div>
                    </div>
                    <Button
                      variant="ghost"
                      size="icon"
                      onClick={() => onDeleteWindow(window.id)}
                    >
                      <Trash2 className="h-4 w-4 text-destructive" />
                    </Button>
                  </div>
                ))}
              </div>
            </div>
          )}

          {/* Add New Window Form */}
          <div className={`space-y-4 pt-4 ${windowDrawer.policy?.windows && windowDrawer.policy.windows.length > 0 ? 'border-t' : ''}`}>
            <Label className="text-base font-semibold">Add Break Window</Label>

            {/* Multi-Day Selection with Checkboxes */}
            <div className="space-y-2">
              <Label>Days of Week</Label>
              <p className="text-xs text-muted-foreground">
                Select which days this break window applies to
              </p>
              <div className="grid grid-cols-2 gap-2 mt-2">
                {DAYS_OF_WEEK.map((day) => (
                  <label key={day.value} className="flex items-center gap-2 cursor-pointer p-2 rounded-md hover:bg-muted/50">
                    <Checkbox
                      checked={windowFormData.selectedDays.includes(day.value)}
                      onCheckedChange={(checked) => {
                        if (checked) {
                          onWindowFormDataChange({
                            ...windowFormData,
                            selectedDays: [...windowFormData.selectedDays, day.value].sort((a, b) => a - b),
                          });
                        } else {
                          onWindowFormDataChange({
                            ...windowFormData,
                            selectedDays: windowFormData.selectedDays.filter((d) => d !== day.value),
                          });
                        }
                      }}
                      disabled={windowDrawer.loading}
                    />
                    <span className="text-sm">{day.label}</span>
                  </label>
                ))}
              </div>
              {/* Quick select buttons */}
              <div className="flex gap-2 mt-2">
                <Button
                  type="button"
                  variant="outline"
                  size="sm"
                  onClick={() => onWindowFormDataChange({ ...windowFormData, selectedDays: [1, 2, 3, 4, 5] })}
                  disabled={windowDrawer.loading}
                >
                  Mon-Fri
                </Button>
                <Button
                  type="button"
                  variant="outline"
                  size="sm"
                  onClick={() => onWindowFormDataChange({ ...windowFormData, selectedDays: [0, 1, 2, 3, 4, 5, 6] })}
                  disabled={windowDrawer.loading}
                >
                  All Days
                </Button>
                <Button
                  type="button"
                  variant="outline"
                  size="sm"
                  onClick={() => onWindowFormDataChange({ ...windowFormData, selectedDays: [] })}
                  disabled={windowDrawer.loading}
                >
                  Clear
                </Button>
              </div>
            </div>

            {/* Time Window */}
            <div className="grid grid-cols-2 gap-4">
              <div className="space-y-2">
                <Label htmlFor="window_start">Window Start</Label>
                <Input
                  id="window_start"
                  type="time"
                  value={windowFormData.window_start}
                  onChange={(e) =>
                    onWindowFormDataChange({ ...windowFormData, window_start: e.target.value })
                  }
                  disabled={windowDrawer.loading}
                />
              </div>
              <div className="space-y-2">
                <Label htmlFor="window_end">Window End</Label>
                <Input
                  id="window_end"
                  type="time"
                  value={windowFormData.window_end}
                  onChange={(e) =>
                    onWindowFormDataChange({ ...windowFormData, window_end: e.target.value })
                  }
                  disabled={windowDrawer.loading}
                />
              </div>
            </div>

            {/* Duration */}
            <div className="grid grid-cols-2 gap-4">
              <div className="space-y-2">
                <Label htmlFor="min_duration">Min Duration (minutes)</Label>
                <Input
                  id="min_duration"
                  type="number"
                  min={1}
                  max={480}
                  value={windowFormData.min_duration_minutes}
                  onChange={(e) =>
                    onWindowFormDataChange({
                      ...windowFormData,
                      min_duration_minutes: Number(e.target.value),
                    })
                  }
                  disabled={windowDrawer.loading}
                />
              </div>
              <div className="space-y-2">
                <Label htmlFor="max_duration">Max Duration (minutes)</Label>
                <Input
                  id="max_duration"
                  type="number"
                  min={1}
                  max={480}
                  value={windowFormData.max_duration_minutes}
                  onChange={(e) =>
                    onWindowFormDataChange({
                      ...windowFormData,
                      max_duration_minutes: Number(e.target.value),
                    })
                  }
                  disabled={windowDrawer.loading}
                />
              </div>
            </div>

            {/* Mandatory with clear explanation */}
            <div className="flex items-center justify-between p-3 bg-muted/30 rounded-md">
              <div className="space-y-0.5">
                <Label>Mandatory Break</Label>
                <p className="text-xs text-muted-foreground max-w-[280px]">
                  {windowDrawer.policy?.tracking_mode === 'auto_deduct'
                    ? 'Break time will be automatically deducted during this window'
                    : 'Employee must take a break during this window'}
                </p>
              </div>
              <Switch
                checked={windowFormData.is_mandatory}
                onCheckedChange={(checked) =>
                  onWindowFormDataChange({ ...windowFormData, is_mandatory: checked })
                }
                disabled={windowDrawer.loading}
              />
            </div>

            <Button
              onClick={onAddWindow}
              disabled={windowDrawer.loading || windowFormData.selectedDays.length === 0}
              className="w-full"
            >
              <Plus className="h-4 w-4 mr-2" />
              {windowDrawer.loading
                ? 'Adding...'
                : windowFormData.selectedDays.length === 0
                  ? 'Select days first'
                  : `Add Window for ${windowFormData.selectedDays.length} day${windowFormData.selectedDays.length !== 1 ? 's' : ''}`}
            </Button>
          </div>
        </div>
      </SheetContent>
    </Sheet>
  );
}

export default WindowManagementDrawer;
